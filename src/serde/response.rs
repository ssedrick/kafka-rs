struct ResponseHeader {
    correlation_id: i32,
}

pub(crate) struct ResponseMessage {
    header: ResponseHeader,
    body: [u8; 1024],
}

impl ResponseMessage {
    pub fn new(correlation_id: i32) -> Self {
        Self {
            header: ResponseHeader { correlation_id },
            body: [0; 1024],
        }
    }
}
