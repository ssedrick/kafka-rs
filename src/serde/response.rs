use serde::Deserialize;

struct ResponseHeader {
    correlation_id: i32,
}

struct ResponseMessage<T: Deserialize> {
    header: ResponseHeader,
    body: T,
}
