use session::game::Session;
use session::game::chunk::Ref;
use shared::protocol::messages::game::context::{GameContextCreateMessage,
    GameContextDestroyMessage, GameMapMovementMessage, GameMapMovementRequestMessage};
use shared::protocol::*;
use shared::protocol::messages::game::context::roleplay::*;
use std::io::{self, Cursor};
use server::SERVER;

impl Session {
    pub fn handle_game_context_create_request<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let ch = match self.current_character.as_ref() {
            Some(ch) => ch,
            None => return Ok(())
        };

        // 176 (noop)
        // 201 (context_destroy)
        // 200: 01 (context_create)
        // 5684: 0a (LifePointsRegenBeginMessage)
        // 220: 84675074 65d3b2572282191e2224dc4651d97ae2 (CurrentMapMessage)
        // 175 (BasicTimeMessage)
        // 500 (CharacterStatsListMessage)

        let mut buf = GameContextDestroyMessage.as_packet().unwrap();

        GameContextCreateMessage {
            context: 1,
        }.as_packet_with_buf(&mut buf).unwrap();

        CurrentMapMessage {
            map_id: ch.map_id(),
            //"65d3b2572282191e2224dc4651d97ae2".to_string(),
            map_key: "bite".to_string(),
        }.as_packet_with_buf(&mut buf).unwrap();

        write!(SERVER, self.base.token, buf);

        Ok(())
    }

    pub fn handle_map_informations_request<'a>(&mut self, _: Ref<'a>, _: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let ch = match self.current_character.as_ref() {
            Some(ch) => ch,
            None => return Ok(())
        };

        let buf = MapComplementaryInformationsDataMessage {
            sub_area_id: VarShort(1),
            map_id: ch.map_id(),
            houses: Vec::new(),
            actors: vec![ch.as_actor().into()],
            interactive_elements: Vec::new(),
            stated_elements: Vec::new(),
            obstacles: Vec::new(),
            fights: Vec::new(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_game_map_movement_request<'a>(&mut self, _: Ref<'a>,
        mut data: Cursor<Vec<u8>>) -> io::Result<()> {

        let ch = match self.current_character.as_ref() {
            Some(ch) => ch,
            None => return Ok(())
        };

        let msg = try!(GameMapMovementRequestMessage::deserialize(&mut data));

        let buf = GameMapMovementMessage {
            key_movements: msg.key_movements,
            actor_id: ch.minimal().id(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_change_map<'a>(&mut self, _: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        let ch = match self.current_character {
            Some(ref ch) => ch,
            None => return Ok(())
        };

        let msg = try!(ChangeMapMessage::deserialize(&mut data));
        //ch.set_map_id(msg.map_id);

        let buf = CurrentMapMessage {
            map_id: msg.map_id,
            //"65d3b2572282191e2224dc4651d97ae2".to_string(),
            map_key: "bite".to_string(),
        }.as_packet().unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }
}
