use session::game::Session;
use session::game::chunk::Chunk;
use std::io::{self, Cursor};
use shared::protocol::*;
use shared::protocol::messages::game::friend::*;
use shared::net::Msg;

pub fn handle_friends_get_list(self_: &mut Session, chunk: &Chunk, _: Cursor<Vec<u8>>)
    -> io::Result<()> {

    if self_.current_character.is_none() {
        return Ok(())
    }

    let buf = FriendsListMessage {
        friends_list: Vec::new(),
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));

    Ok(())
}

pub fn handle_ignored_get_list(self_: &mut Session, chunk: &Chunk, _: Cursor<Vec<u8>>)
    -> io::Result<()> {

    if self_.current_character.is_none() {
        return Ok(())
    }

    let buf = IgnoredListMessage {
        ignored_list: Vec::new(),
    }.as_packet().unwrap();

    send!(chunk, Msg::Write(self_.token, buf));

    Ok(())
}
