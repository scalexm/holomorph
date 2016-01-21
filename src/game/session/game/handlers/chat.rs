use session::game::{Session, GameState};
use session::game::chunk::{self, Ref};
use protocol::*;
use protocol::messages::game::chat::channel::*;
use protocol::messages::game::chat::*;
use protocol::messages::game::chat::smiley::*;
use protocol::types::game::data::items::ObjectItem;
use protocol::messages::game::basic::{BasicNoOperationMessage, TextInformationMessage};
use protocol::enums::{text_information_type, chat_channels_multi};
use time;
use std::io::Result;
use server::{social, SERVER};

macro_rules! build_message {
    ($msg: ident, $items: ident) => {
        if $items.len() > 0 {
            ChatServerWithObjectMessage {
                base: $msg,
                objects: $items,
            }.unwrap()
        }
        else {
            $msg.unwrap()
        }
    };
}

impl Session {
    fn send_chat_message<'a>(&mut self, mut chunk: Ref<'a>, msg: ChatClientMultiMessage,
                             items: Vec<ObjectItem>) {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return,
        };

        if !self.account.as_ref().unwrap().channels.contains(&(msg.channel as u8)) {
            let buf = BasicNoOperationMessage.unwrap();
            write!(SERVER, self.base.token, buf);
            return;
        }

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
                sender_id: ch.minimal().id() as f64,
                sender_name: ch.minimal().name().to_string(),
                sender_account_id: self.account.as_ref().unwrap().id,
            }
        };

        let buf = build_message!(resp, items);

        if msg.channel == chat_channels_multi::GLOBAL {
            chunk.maps.get(&map_id).unwrap().send_only_to(buf, |ch| ch.has_global_channel());
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
                }.unwrap();
                write!(SERVER, self.base.token, buf);
                return ();
            }

            *last_request = now;

            let area_id = chunk.maps.get(&map_id).unwrap().area_id();
            chunk.eventually(move |chunk| chunk::send_to_area(chunk, area_id, buf));
        }
    }

    fn send_private_message<'a>(&self, chunk: Ref<'a>, msg: ChatClientPrivateMessage,
                                items: Vec<ObjectItem>) {
        let ch = match self.state {
            GameState::InContext(ref ch) => get_character!(ch, chunk),
            _ => return,
        };

        SERVER.with(move |s| {
            social::send_private_message(
                &s.server,
                self.base.token,
                self.account.as_ref().unwrap().id,
                ch.minimal().name().to_string(),
                ch.minimal().id(),
                msg.receiver,
                msg.base.content,
                items
            )
        });
    }
}

#[register_handlers]
impl Session {
    pub fn handle_channel_enabling<'a>(&mut self, mut chunk: Ref<'a>, msg: ChannelEnablingMessage)
                                        -> Result<()> {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return Ok(()),
        };

        let _ = {
            let account = self.account.as_mut().unwrap();
            if msg.enable {
                let _ = account.channels.insert(msg.channel as u8);
            } else {
                let _ = account.channels.remove(&(msg.channel as u8));
            }
        };

        if msg.channel == chat_channels_multi::GLOBAL {
            get_mut_character!(ch, chunk).set_has_global_channel(msg.enable);
        }

        let buf = ChannelEnablingChangeMessage {
            channel: msg.channel,
            enable: msg.enable,
        }.unwrap();

        write!(SERVER, self.base.token, buf);
        Ok(())
    }

    pub fn handle_chat_client_multi<'a>(&mut self, chunk: Ref<'a>, msg: ChatClientMultiMessage)
                                        -> Result<()> {
        self.send_chat_message(chunk, msg, Vec::new());
        Ok(())
    }

    pub fn handle_chat_client_multi_with_object<'a>(&mut self, chunk: Ref<'a>,
                                                    msg: ChatClientMultiWithObjectMessage)
                                                    -> Result<()> {
        self.send_chat_message(chunk, msg.base, msg.objects);
        Ok(())
    }

    pub fn handle_chat_client_private<'a>(&mut self, chunk: Ref<'a>,
                                          msg: ChatClientPrivateMessage) -> Result<()> {
        self.send_private_message(chunk, msg, Vec::new());
        Ok(())
    }

    pub fn handle_chat_client_private_with_object<'a>(&mut self, chunk: Ref<'a>,
                                                      msg: ChatClientPrivateWithObjectMessage)
                                                      -> Result<()> {
        self.send_private_message(chunk, msg.base, msg.objects);
        Ok(())
    }

    pub fn handle_chat_smiley_request<'a>(&mut self, chunk: Ref<'a>,
                                      msg: ChatSmileyRequestMessage) -> Result<()> {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return Ok(()),
        };

        let buf = ChatSmileyMessage {
            entity_id: ch.id as f64,
            smiley_id: msg.smiley_id,
            account_id: self.account.as_ref().unwrap().id,
        }.unwrap();
        chunk.maps.get(&ch.map_id).unwrap().send(buf);

        Ok(())
    }

    pub fn handle_mood_smiley_request<'a>(&mut self, mut chunk: Ref<'a>,
                                          msg: MoodSmileyRequestMessage) -> Result<()> {
        let ch = match self.state {
            GameState::InContext(ref ch) => ch,
            _ => return Ok(()),
        };

        let smiley = msg.smiley_id.0;
        get_mut_character!(ch, chunk).set_mood_smiley(smiley);
        SERVER.with(|s| social::update_mood(&s.server, ch.id, smiley));

        let buf = MoodSmileyResultMessage {
            result_code: 0,
            smiley_id: msg.smiley_id,
        }.unwrap();
        write!(SERVER, self.base.token, buf);
        Ok(())
    }
}
