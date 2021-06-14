use crate::serde::ApiKey;
use serde::Serialize;

pub(crate) struct RequestHeader<'a> {
    api_key: ApiKey,
    api_version: i16,
    correlation_id: i32,
    client_id: Option<&'a str>,
}

pub(crate) struct RequestMessage<'a, T: Serialize> {
    header: RequestHeader<'a>,
    body: T,
}
