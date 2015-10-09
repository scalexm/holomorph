use session::game::{GameState, Session, CharacterRef};
use session::game::chunk::{self, Ref};
use shared::protocol::messages::game::context::{GameContextCreateMessage,
    GameContextDestroyMessage, GameMapMovementRequestMessage, GameMapMovementCancelMessage};
use shared::protocol::*;
use shared::protocol::messages::game::context::roleplay::*;
use shared::protocol::messages::game::basic::{TextInformationMessage, BasicNoOperationMessage};
use shared::protocol::enums::text_information_type;
use std::io::{Result, Cursor};
use server::SERVER;
use time;
use std::mem;

impl Session {
    pub fn handle_game_context_create_request<'a>(&mut self, mut chunk: Ref<'a>,
        _: Cursor<Vec<u8>>) -> Result<()> {

        let (map_id, ch_id) = match self.state {
            GameState::SwitchingContext(map_id, ref ch) => (map_id, ch.minimal().id()),
            _ => return Ok(()),
        };

        let tok = self.base.token;

        if !SERVER.with(|s| s.maps.contains_key(&map_id)) {
            log_err!(self, "context_create: map not found {}", map_id);
            close!(SERVER, tok);
            return Ok(());
        }

        let state = mem::replace(&mut self.state, GameState::InContext(CharacterRef {
            id: ch_id,
            map_id: map_id,
            movements: None,
        }));

        let ch = match state {
            GameState::SwitchingContext(_, ch) => ch,
            _ => unreachable!(),
        };

        let cell_id = ch.cell_id();
        chunk.eventually(move |chunk| {
            chunk::teleport_character(chunk, ch, map_id, cell_id)
        });

        let mut buf = GameContextDestroyMessage.as_packet().unwrap();

        GameContextCreateMessage {
            context: 1,
        }.as_packet_with_buf(&mut buf).unwrap();

        TextInformationMessage {
            msg_type: text_information_type::ERROR,
            msg_id: VarShort(89),
            parameters: Vec::new(),
        }.as_packet_with_buf(&mut buf).unwrap();

        let last_connection = self.account.as_ref().unwrap().last_connection;
        if last_connection != 0 {
            let tm = time::at(time::Timespec {
                sec: last_connection,
                nsec: 0,
            });

            let mut minutes = tm.tm_min.to_string();
            if tm.tm_min < 10 {
                minutes = minutes + "0";
            }
            TextInformationMessage {
                msg_type: text_information_type::MESSAGE,
                msg_id: VarShort(152),
                parameters: vec![(1900 + tm.tm_year).to_string(), (1 + tm.tm_mon).to_string(),
                    tm.tm_mday.to_string(), tm.tm_hour.to_string(), minutes,
                    self.account.as_ref().unwrap().last_ip.clone()],
            }.as_packet_with_buf(&mut buf).unwrap();
        }

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_map_informations_request<'a>(&mut self, chunk: Ref<'a>, _: Cursor<Vec<u8>>)
        -> Result<()> {

        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return Ok(()),
        };

        let map = chunk.maps.get(&ch.map_id).unwrap();
        let buf = map
            .get_complementary_informations()
            .as_packet()
            .unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_game_map_movement_request<'a>(&mut self, chunk: Ref<'a>,
        mut data: Cursor<Vec<u8>>) -> Result<()> {

        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        if ch.movements.is_some() {
            return Ok(());
        }

        let msg = try!(GameMapMovementRequestMessage::deserialize(&mut data));

        ch.movements = Some(msg.key_movements.clone());
        chunk.maps.get(&ch.map_id).unwrap().start_move_actor(ch.id, msg.key_movements);

        Ok(())
    }

    pub fn handle_game_map_movement_confirm<'a>(&mut self, mut chunk: Ref<'a>,
        _: Cursor<Vec<u8>>) -> Result<()> {

        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        let movements = mem::replace(&mut ch.movements, None);
        let movements = match movements {
            Some(movements) => movements,
            None => return Ok(()),
        };

        let last_mov = movements[movements.len() - 1];
        let new_cell = last_mov & 4095;
        let new_dir = ((new_cell ^ last_mov) >> 12) as i8;

        let ch = get_mut_character!(ch, chunk);
        ch.set_cell_id(new_cell);
        ch.set_direction(new_dir);

        write!(SERVER, self.base.token, BasicNoOperationMessage.as_packet().unwrap());

        Ok(())
    }

    pub fn handle_game_map_movement_cancel<'a>(&mut self, mut chunk: Ref<'a>,
        mut data: Cursor<Vec<u8>>) -> Result<()> {

        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        let msg = try!(GameMapMovementCancelMessage::deserialize(&mut data));

        let movements = mem::replace(&mut ch.movements, None);
        let movements = match movements {
            Some(movements) => movements,
            None => return Ok(()),
        };

        chunk.maps.get(&ch.map_id).unwrap().end_move_actor(ch.id);

        let ch = get_mut_character!(ch, chunk);
        ch.set_cell_id(msg.cell_id.0);

        Ok(())
    }

    pub fn handle_change_map<'a>(&mut self, chunk: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> Result<()> {

        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        if ch.movements.is_some() {
            return Ok(());
        }

        let msg = try!(ChangeMapMessage::deserialize(&mut data));
        let cell = get_character!(ch, chunk).cell_id();

        let new_pos = SERVER.with(|s| {
            let map = s.maps.get(&ch.map_id).unwrap();
            let (_, change_data) = map.get_cell_data(cell);

            if change_data == 0 {
                return None;
            }

            let id = msg.map_id;
            let offset = match cell {
                0 | 14 => if id == map.client_top() { 64 } else { 16 },
                545 | 559 => if id == map.client_bottom() { 4 } else { 1 },

                1 ... 14 | 15 ... 27 => 64,
                533 ... 545 | 546 ... 559 => 4,
                _ if cell % 14 == 0 => 16,
                _ if (cell + 1) % 14 == 0 => 1,

                _ => return None,
            };

            let (new_map, cell_add, cell_add_left, cell_add_right, custom_cell) = match offset {
                1 => (map.right(), -13, 1, -27, map.custom_right_cell()),
                4 => (map.bottom(), -532, -545, -546, map.custom_bottom_cell()),
                16 => (map.left(), 13, -1, 27, map.custom_left_cell()),
                64 => (map.top(), 532, 546, 545, map.custom_top_cell()),
                _ => unreachable!(),
            };

            if custom_cell >= 0 && custom_cell <= 559 {
                return Some((new_map, custom_cell));
            }

            let left_offset = 2 * offset;
            let right_offset = offset / 2 + if offset == 1 { 128 } else { 0 };

            Some((new_map, cell + if change_data & offset == offset {
                cell_add
            } else if change_data & right_offset == right_offset {
                cell_add_right
            } else if change_data & left_offset == left_offset {
                cell_add_left
            } else {
                return None
            }))
        });

        if let Some((new_map, new_cell)) = new_pos {
            chunk::teleport(chunk, ch, new_map, new_cell);
        }

        Ok(())
    }
}
