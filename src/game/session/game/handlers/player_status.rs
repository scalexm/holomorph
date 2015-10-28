use protocol::*;
use protocol::messages::game::character::status::*;
use protocol::variants::PlayerStatusVariant;
use protocol::enums::player_status;
use session::game::{GameState, Session};
use session::game::chunk::Ref;
use std::io::{Result, Cursor};
use server::{social, SERVER};

impl Session {
    pub fn handle_player_status_update_request<'a>(&mut self, _: Ref<'a>,
                                                   mut data: Cursor<Vec<u8>>) -> Result<()> {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return Ok(()),
       };

       let mut msg = try!(PlayerStatusUpdateRequestMessage::deserialize(&mut data));
       if let PlayerStatusVariant::PlayerStatusExtended(ref mut status) = msg.status {
           if status.base.status_id != player_status::AFK || status.message.len() > 200 {
               let buf = PlayerStatusUpdateErrorMessage.as_packet().unwrap();
               write!(SERVER, self.base.token, buf);
               return Ok(());
           }
           status.message = status.message.replace("<", "&lt;");
           status.message = status.message.replace(">", "&gt;");
       }

       self.account.as_mut().unwrap().social.status = msg.status.clone();
       let account = self.account.as_ref().unwrap();

       let buf = PlayerStatusUpdateMessage {
           account_id: account.id,
           player_id: VarInt(ch.id),
           status: msg.status.clone(),
       }.as_packet().unwrap();
       write!(SERVER, self.base.token, buf);

       let status = msg.status;
       SERVER.with(|s| social::update_player_status(&s.server, account.id, ch.id, status));
       Ok(())
    }
}
