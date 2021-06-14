pub mod request;
pub mod response;

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum ApiKey {
    Produce = 0,
    Fetch,
    _ListOffsets,
    Metadata,
    _LeaderAndIsr,
    _StopReplica,
    _UpdateMetadata,
    _ControlledShutdown,
    _OffsetCommit,
    _OffsetFetch,
    _FindCoordinator,
    _JoinGroup,
    _Heartbeat,
    _LeaveGroup,
    _SyncGroup,
    _DescribeGroups,
    _ListGroups,
    _SaslHandshake,
    ApiVerions,
}

impl ApiKey {
    pub fn to_code(api_key: ApiKey) -> i16 {
        api_key as i16
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Acks {
    FullIsr = -1,
    NoAck = 0,
    Leader = 1,
}

impl Acks {
    pub fn to_code(ack: Acks) -> i16 {
        ack as i16
    }
}
