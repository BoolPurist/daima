use serde::{Deserialize, Serialize};

use crate::{
    binary_generation,
    binary_parsing::ParsedBinaryStream,
    binary_type_marshalling::{self, SerilizedAsBinaries},
    message::{InitMessage, UnkownMessage},
    AppResult, TypeTagSize,
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
    pub fn tag_code(&self) -> TypeTagSize {
        match self {
            IpcMessage::Init(_) => 0,
            IpcMessage::UnkownMessage(_) => 1,
        }
    }

    pub fn to_bytes(&self) -> SerilizedAsBinaries {
        let type_tag = binary_generation::u16_to_bytes(self.tag_code());
        let as_bytes = binary_type_marshalling::serilize(self)?;
        let mut prefix_length = binary_generation::size_to_bytes_sequence(as_bytes.len());
        prefix_length.extend_from_slice(&type_tag);
        prefix_length.extend_from_slice(&as_bytes);
        Ok(prefix_length)
    }

    pub fn from_bytes(buffer: ParsedBinaryStream) -> AppResult<Self> {
        let payload = buffer.payload_as_bytes();
        match buffer.type_tag() {
            0 => Self::parse_it(payload),
            _ => Ok(Self::UnkownMessage(UnkownMessage)),
        }
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
        if let ByteStreamReadingState::Done { parsed, rest } = back_from_bytes {
            assert!(rest.is_empty());
            let back_from_bytes = IpcMessage::from_bytes(parsed).unwrap();
            assert_eq!(given, back_from_bytes);
        } else {
            panic!()
        }
    }
}
