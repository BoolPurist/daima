// TODO: remove allow dead_code when used
#![allow(dead_code)]

use std::usize;

use crate::constants::{NEXT_NUMBER_OF_BYTES_MASK, SHIFT_NUMBER_OF_BYTE_PAYLOAD};

type NumberBytesSize = usize;

const CONTINUEATION_BYTE_MASK: NumberBytesSize = 0b1000_0000;
const AFTER_NEXT_SINGLE_BYTE: NumberBytesSize = 1;
const NEXT_SINGLE_BYTE: NumberBytesSize = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedBinaryStream {
    number_of_bytes_payload: NumberBytesSize,
    payload_as_bytes: Vec<u8>,
}

impl ParsedBinaryStream {
    pub fn number_of_bytes_payload(&self) -> usize {
        self.number_of_bytes_payload
    }

    pub fn payload_as_bytes(&self) -> &[u8] {
        &self.payload_as_bytes
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ByteStreamReadingState<'s> {
    Length {
        shift_counter: NumberBytesSize,
        current_number_of_bytes: NumberBytesSize,
    },
    PayLoad(ParsedBinaryStream),
    Done {
        parsed: ParsedBinaryStream,
        rest: &'s [u8],
    },
}

impl<'s> ByteStreamReadingState<'s> {
    pub fn start(stream: &'s [u8]) -> Self {
        let not_started = ByteStreamReadingState::Length {
            shift_counter: 0,
            current_number_of_bytes: 0,
        };
        not_started.advance(stream)
    }

    fn advance(self, stream: &'s [u8]) -> Self {
        let mut current_stream = stream;
        match self {
            ByteStreamReadingState::Length {
                mut shift_counter,
                mut current_number_of_bytes,
            } => {
                while !current_stream.is_empty() {
                    // u8 is always large enought for a usize for safe casting
                    let next_byte = current_stream[NEXT_SINGLE_BYTE] as NumberBytesSize;
                    current_stream = &current_stream[AFTER_NEXT_SINGLE_BYTE..];

                    let (is_last_byte, to_append) = (
                        (next_byte & CONTINUEATION_BYTE_MASK) != CONTINUEATION_BYTE_MASK,
                        next_byte & NEXT_NUMBER_OF_BYTES_MASK,
                    );
                    current_number_of_bytes |=
                        to_append << (shift_counter * SHIFT_NUMBER_OF_BYTE_PAYLOAD);
                    if is_last_byte {
                        return ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                            number_of_bytes_payload: current_number_of_bytes,
                            payload_as_bytes: Vec::with_capacity(current_number_of_bytes),
                        })
                        .advance(current_stream);
                    }
                    shift_counter += 1;
                }
                ByteStreamReadingState::Length {
                    shift_counter,
                    current_number_of_bytes,
                }
            }
            ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                number_of_bytes_payload,
                mut payload_as_bytes,
            }) => {
                let next_bytes_len = current_stream.len();
                let left_bytes_to_read = number_of_bytes_payload - payload_as_bytes.len();
                let next_slice_upper_bound = next_bytes_len.min(left_bytes_to_read);
                let next_bytes = &current_stream[NEXT_SINGLE_BYTE..next_slice_upper_bound];
                current_stream = &current_stream[next_slice_upper_bound..];
                payload_as_bytes.extend_from_slice(next_bytes);
                if payload_as_bytes.len() == number_of_bytes_payload {
                    return ByteStreamReadingState::Done {
                        parsed: ParsedBinaryStream {
                            number_of_bytes_payload,
                            payload_as_bytes,
                        },
                        rest: current_stream,
                    };
                }

                ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                    number_of_bytes_payload,
                    payload_as_bytes,
                })
            }
            ByteStreamReadingState::Done { parsed, rest } => {
                return ByteStreamReadingState::Done { parsed, rest };
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::binary_parsing::ParsedBinaryStream;

    use super::ByteStreamReadingState;

    #[test]
    fn parse_empty_at_start() {
        let input: &[u8] = &[];
        let actual = ByteStreamReadingState::start(input);
        let expected = ByteStreamReadingState::Length {
            shift_counter: 0,
            current_number_of_bytes: 0,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_number_of_length_of_byte_stream() {
        fn assert_case(input: &[u8], expected: ByteStreamReadingState) {
            let actual = ByteStreamReadingState::start(input);
            assert_eq!(expected, actual, "Input {:?}\n", input);
        }
        // 000_0010 | 000_0010
        // 00000_001 0000_0010
        assert_case(
            &[0b1000_0010, 0b1000_0010],
            ByteStreamReadingState::Length {
                shift_counter: 2,
                current_number_of_bytes: 258,
            },
        );
        assert_case(
            &[0b1001_0101],
            ByteStreamReadingState::Length {
                shift_counter: 1,
                current_number_of_bytes: 21,
            },
        );
        assert_case(
            &[0b0001_0101, 0b0101],
            ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                number_of_bytes_payload: 21,
                payload_as_bytes: vec![0b0101],
            }),
        );
        assert_case(
            &[
                // number of bytes
                0b0000_0011,
                // payload
                0b1,
                0b11,
                0b111,
                // rest
                0b1111_1111,
            ],
            ByteStreamReadingState::Done {
                parsed: ParsedBinaryStream {
                    number_of_bytes_payload: 3,
                    payload_as_bytes: vec![0b1, 0b11, 0b111],
                },
                rest: &[0b1111_1111],
            },
        );
    }
}
