use serde::{Deserialize, Serialize};

use crate::{
    binary_generation,
    binary_parsing::ParsedBinaryStream,
    binary_type_marshalling::{self, SerilizedAsBinaries},
    message::{InitMessage, UnkownMessage},
    AppResult,
};

#[derive(Serialize, PartialEq, Eq, Debug, Deserialize)]
pub enum IpcMessage {
    Init(InitMessage),
    UnkownMessage(UnkownMessage),
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum InvalidBytesForMessage {
    #[error("Given bytes for tags not valid {0:?}")]
    TagMessage([u8; 2]),
    #[error("Payload is in an invalid format for the given message with the tag {0:?}")]
    FormatMessage([u8; 2]),
}

impl IpcMessage {
    pub fn to_bytes(&self) -> SerilizedAsBinaries {
        let as_bytes = binary_type_marshalling::serilize(self)?;
        let mut prefix_length = binary_generation::size_to_bytes_sequence(as_bytes.len());
        prefix_length.extend_from_slice(&as_bytes);
        Ok(prefix_length)
    }

    pub fn from_bytes(buffer: &ParsedBinaryStream) -> AppResult<Self> {
        let payload = buffer.payload_as_bytes();
        Self::parse_it(payload)
    }

    fn parse_it(value: &[u8]) -> AppResult<Self> {
        let parsed = binary_type_marshalling::deserilize(value)?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod testing {
    use core::panic;

    use crate::{binary_parsing::ByteStreamReadingState, message::InitMessage};

    use super::IpcMessage;

    #[test]
    fn messages_to_bytes_and_back_to_message() {
        let given = IpcMessage::Init(InitMessage::new("some_application".to_string()));
        let as_bytes = given.to_bytes().unwrap();
        let back_from_bytes = ByteStreamReadingState::start(&as_bytes);
        if let ByteStreamReadingState::Done { parsed, rest } = &back_from_bytes {
            assert!(rest.is_empty());
            let back_from_bytes = IpcMessage::from_bytes(parsed).unwrap();
            assert_eq!(given, back_from_bytes);
        } else {
            panic!("Actual parsing state: {:?}", back_from_bytes)
        }
    }
}
