use crate::{NumberBytesSize, TypeTagSize};

pub const APP_DAEMON_NAME: &str = "daima_deamon";

pub(crate) const SHIFT_AMOUNT_TYPETAG: TypeTagSize = 8;
pub(crate) const NUMBER_OF_TYPE_TAG_BYTES: TypeTagSize = 2;
pub(crate) const SHIFT_NUMBER_OF_BYTE_PAYLOAD: NumberBytesSize = 7;
pub(crate) const NEXT_NUMBER_OF_BYTES_MASK: NumberBytesSize = 0b0111_1111;
pub(crate) const CONTINUEATION_BYTE_MASK: NumberBytesSize = 0b1000_0000;
pub(crate) const ONE_BYTE_BIT_MASK: u8 = 0b1111_1111;
