pub(crate) enum KafkaError {
    UnknownServerError,
    OffsetOutOfRange,
    // Fill in more
}

impl KafkaError {
    pub fn from_code(code: i16) -> Option<KafkaError> {
        match code {
            0 => None,
            1 => Some(KafkaError::OffsetOutOfRange),
            _ => Some(KafkaError::UnknownServerError),
        }
    }
}
