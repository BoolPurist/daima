use crate::NumberBytesSize;

pub const APP_DAEMON_NAME: &str = "daima_deamon";

pub(crate) const SHIFT_NUMBER_OF_BYTE_PAYLOAD: NumberBytesSize = 7;
pub(crate) const NEXT_NUMBER_OF_BYTES_MASK: NumberBytesSize = 0b0111_1111;
pub(crate) const CONTINUEATION_BYTE_MASK: NumberBytesSize = 0b1000_0000;
