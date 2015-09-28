use session::game::{Session, GameState};
use session::game::chunk::{self, Ref};
use shared::protocol::*;
use shared::protocol::messages::game::chat::*;
use shared::protocol::types::game::data::items::ObjectItem;
use shared::protocol::messages::game::basic::TextInformationMessage;
use shared::protocol::enums::{text_information_type, chat_channels_multi};
use time;
use std::io::{self, Cursor};
use server::SERVER;

macro_rules! build_message {
    ($msg: ident, $items: ident) => {
        if $items.len() > 0 {
            ChatServerWithObjectMessage {
                base: $msg,
                objects: $items,
            }.as_packet().unwrap()
        }
        else {
            $msg.as_packet().unwrap()
        }
    };
}

impl Session {
    fn send_chat_message<'a>(&mut self, mut chunk: Ref<'a>, msg: ChatClientMultiMessage,
        items: Vec<ObjectItem>) {

        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => unreachable!(),
        };

        let map_id = ch.map_id;

        let resp = {
            let ch = get_character!(ch, chunk);

            ChatServerMessage {
                base: ChatAbstractServerMessage {
                    channel: msg.channel,
                    content: msg.base.content,
                    timestamp: time::get_time().sec as i32,
                    fingerprint: String::new(),
                },
                sender_id: ch.minimal().id(),
                sender_name: ch.minimal().name().to_string(),
                sender_account_id: self.account.as_ref().unwrap().id,
            }
        };

        let buf = build_message!(resp, items);

        if msg.channel == chat_channels_multi::GLOBAL {
            chunk.maps.get(&map_id).unwrap().send(buf);
        } else if msg.channel == chat_channels_multi::SALES
            || msg.channel == chat_channels_multi::SEEK {

            let last_request = if msg.channel == chat_channels_multi::SALES {
                &mut self.last_sales_chat_request
            } else {
                &mut self.last_seek_chat_request
            };

            let now = time::get_time().sec;

            if now < *last_request + 60 {
                let buf = TextInformationMessage {
                    msg_type: text_information_type::MESSAGE,
                    msg_id: VarShort(115),
                    parameters: vec![(*last_request + 60 - now).to_string()],
                }.as_packet().unwrap();
                write!(SERVER, self.base.token, buf);
                return ();
            }

            *last_request = now;

            let area_id = chunk.maps.get(&map_id).unwrap().area_id();
            chunk.eventually(move |chunk| chunk::send_to_area(chunk, area_id, buf));
        }
    }

    pub fn handle_chat_client_multi<'a>(&mut self, chunk: Ref<'a>, mut data: Cursor<Vec<u8>>)
        -> io::Result<()> {

        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let msg = try!(ChatClientMultiMessage::deserialize(&mut data));

        self.send_chat_message(chunk, msg, Vec::new());
        Ok(())
    }

    pub fn handle_chat_client_multi_with_object<'a>(&mut self, chunk: Ref<'a>,
        mut data: Cursor<Vec<u8>>) -> io::Result<()> {

        match self.state {
            GameState::InContext(_) => (),
            _ => return Ok(()),
        };

        let msg = try!(ChatClientMultiWithObjectMessage::deserialize(&mut data));

        self.send_chat_message(chunk, msg.base, msg.objects);
        Ok(())
    }
}
