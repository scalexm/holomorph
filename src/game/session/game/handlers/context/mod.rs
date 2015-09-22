use session::game::{GameState, Session, CharacterRef};
use session::game::chunk::{self, Ref};
use shared::protocol::messages::game::context::{GameContextCreateMessage,
    GameContextDestroyMessage, GameMapMovementRequestMessage, GameMapMovementCancelMessage};
use shared::protocol::*;
use shared::protocol::messages::game::context::roleplay::*;
use shared::protocol::messages::game::basic::BasicNoOperationMessage;
use std::io::{self, Cursor};
use server::SERVER;
use std::mem;

impl Session {
    pub fn handle_game_context_create_request<'a>(&mut self, mut chunk: Ref<'a>,
        _: Cursor<Vec<u8>>) -> io::Result<()> {

        let (map_id, ch_id) = match self.state {
            GameState::SwitchingContext(map_id, ref ch) => (map_id, ch.minimal().id()),
            _ => return Ok(()),
        };

        let tok = self.base.token;

        if !SERVER.with(|s| s.maps.contains_key(&map_id)) {
            error!("context_create: map not found {}", map_id);
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

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_map_informations_request<'a>(&mut self, chunk: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

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
        mut data: Cursor<Vec<u8>>) -> io::Result<()> {

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
        _: Cursor<Vec<u8>>) -> io::Result<()> {

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
        mut data: Cursor<Vec<u8>>) -> io::Result<()> {

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
        -> io::Result<()> {

        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        if ch.movements.is_some() {
            return Ok(());
        }

        let msg = try!(ChangeMapMessage::deserialize(&mut data));
        let cell = get_character!(ch, chunk).cell_id();

        let new_cell = SERVER.with(|s| {
            let map = s.maps.get(&ch.map_id).unwrap();
            match msg.map_id {
                id if id == map.left() => Some(cell + 13),
                id if id == map.right() => Some(cell - 13),
                id if id == map.bottom() => Some(cell - 532),
                id if id == map.top() => Some(cell + 532),
                _ => None,
            }
        });

        if let Some(new_cell) = new_cell {
            let _ = chunk::teleport(chunk, ch, msg.map_id, new_cell);
        }

        Ok(())
    }
}
