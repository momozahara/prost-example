use std::convert::TryFrom;

use user::User;

use crate::user::user::{Weight, WeightOption};

#[cfg(test)]
mod lib_test;

// Include the `bar` module, which is generated from bar.proto.
pub mod user {
    include!(concat!(env!("OUT_DIR"), "/user.rs"));
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.first_name,
            self.last_name,
            match self.age.as_ref() {
                Some(age) => age.to_string(),
                None => "None".to_string(),
            },
            match self.weight_option {
                Some(WeightOption::Weight(weight)) => {
                    Weight::try_from(weight).unwrap().as_str_name()
                }
                _ => "None",
            }
        )
    }
}

pub fn create_user(
    first_name: String,
    last_name: String,
    age: Option<i32>,
    weight: Option<WeightOption>,
) -> User {
    let user = User {
        first_name,
        last_name,
        age,
        weight_option: weight,
    };
    user
}
