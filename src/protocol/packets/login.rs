use crate::{packets};
use crate::protocol::fields::{VarString};

packets! {
    LoginStart (0x00, C, Login) {
        name: VarString
    },
}