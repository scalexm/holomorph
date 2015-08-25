use shared::protocol::*;
use shared::protocol::messages::connection::*;
use std::io::Cursor;

#[test]
fn test_protocol() {
    let msg = IdentificationSuccessMessage {
        has_rights: Flag(true),
        was_already_connected: Flag(false),
        login: "salut".to_string(),
        nickname: "yo".to_string(),
        account_id: 54,
        community_id: 1,
        secret_question: "hello".to_string(),
        account_creation: 64.,
        subscription_elapsed_duration: 74.,
        subscription_end_date: 84.,
    };

    let mut buf = Vec::new();
    msg.serialize(&mut buf).unwrap();
    let mut buf = Cursor::new(buf);
    let msg = IdentificationSuccessMessage::deserialize(&mut buf).unwrap();

    assert!(msg.has_rights.0 && !msg.was_already_connected.0);
    assert_eq!("yo", &msg.nickname);
    assert_eq!(1, msg.community_id);
    assert_eq!(74., msg.subscription_elapsed_duration);
}
