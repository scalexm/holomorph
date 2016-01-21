use protocol::*;
use protocol::messages::game::character::status::*;
use protocol::variants::PlayerStatusVariant;
use protocol::enums::player_status;
use session::game::{GameState, Session};
use session::game::chunk::Ref;
use std::io::Result;
use server::{social, SERVER};

impl Session {
    pub fn update_status(&mut self, mut status: PlayerStatusVariant) {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return,
        };

        if let PlayerStatusVariant::PlayerStatusExtended(ref mut status) = status {
            if status.base.status_id != player_status::AFK || status.message.len() > 200 {
                let buf = PlayerStatusUpdateErrorMessage.unwrap();
                write!(SERVER, self.base.token, buf);
                return;
            }
            status.message = status.message.replace("<", "&lt;");
            status.message = status.message.replace(">", "&gt;");
        }

        self.account.as_mut().unwrap().social.status = status.clone();
        let account = self.account.as_ref().unwrap();

        let buf = PlayerStatusUpdateMessage {
            account_id: account.id,
            player_id: VarLong(ch.id),
            status: status.clone(),
        }.unwrap();
        write!(SERVER, self.base.token, buf);

        SERVER.with(|s| social::update_player_status(&s.server, account.id, ch.id, status));
    }
}

#[register_handlers]
impl Session {
    pub fn handle_player_status_update_request<'a>(&mut self, _: Ref<'a>,
                                                   msg: PlayerStatusUpdateRequestMessage)
                                                   -> Result<()> {
        self.update_status(msg.status);
        Ok(())
    }
}
