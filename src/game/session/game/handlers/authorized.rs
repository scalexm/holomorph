use session::game::{Session, GameState};
use session::game::chunk::{self, Ref};
use protocol::messages::authorized::*;
use std::io::Result;
use server::SERVER;

#[register_handlers]
impl Session {
    pub fn handle_admin_quiet_command_message<'a>(&mut self, chunk: Ref<'a>,
                                                  msg: AdminQuietCommandMessage) -> Result<()> {
        let ch = match self.state {
            GameState::InContext(ref mut ch) => ch,
            _ => return Ok(()),
        };

        if ch.movements.is_some() {
            return Ok(());
        }

        let map_id: i32 = match msg.base.content.split(" ").last().map(|id| id.parse()) {
            Some(Ok(map_id)) => map_id,
            _ => return Ok(()),
        };

        let cell_id = SERVER.with(|s| {
            s.maps.get(&map_id).map(|m| m.get_free_cell()).unwrap_or(None)
        });

        if let Some(cell_id) = cell_id {
            chunk::teleport(chunk, ch, map_id, cell_id);
        }
        Ok(())
    }
}
