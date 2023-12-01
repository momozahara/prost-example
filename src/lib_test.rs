use prost::Message;

use crate::{
    create_user,
    user::{
        user::{Weight, WeightOption},
        User,
    },
};

fn create_user_t(weight: Option<WeightOption>) -> User {
    create_user("John".to_owned(), "Doe".to_owned(), None, weight)
}

#[test]
fn test_create_user() {
    let user = create_user_t(Some(WeightOption::Weight(Weight::Heavy as i32)));
    let user2 = create_user_t(Some(WeightOption::NoWeight(())));

    assert_eq!(format!("{user}"), "John Doe None HEAVY");
    assert_eq!(format!("{user2}"), "John Doe None None");
}

#[test]
fn test_encode_decode_user() {
    let user = create_user_t(None);
    let mut buf = Vec::new();
    buf.reserve(user.encoded_len());
    user.encode(&mut buf).unwrap();

    let user = User::decode(&mut std::io::Cursor::new(buf.as_slice()).into_inner()).unwrap();
    let mut buf2 = Vec::new();
    buf.reserve(user.encoded_len());
    user.encode(&mut buf2).unwrap();

    assert_eq!(buf, buf2);
}
