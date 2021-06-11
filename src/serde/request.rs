use serde::Serialize;

pub(crate) struct RequestHeader {
    api_key: ApiKey,
    api_version: i16,
    correlation_id: i32,
    client_id: Option<&str>,
}

enum Request<T: Serialize> {}

pub(crate) struct RequestMessage<T: Serialize> {
    header: RequestHeader,
    body: Request<T>,
}
