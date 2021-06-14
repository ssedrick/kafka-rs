use crate::client::Client;
use crate::serde::{response::ResponseMessage, Acks, ApiKey};

struct Header<'d, 'e> {
    key: &'d str,
    value: &'e [u8]
}

struct Record<'a, 'b, 'c, 'd, 'e> {
    attributes: i8,
    // VarLong
    timestamp_delta: i64,
    // VarInt
    offset_delta: i32,
    key: &'a [u8],
    value: &'b [u8],
    headers: &'c [Header<'d, 'e>],
}

struct RecordBatch {
    base_offset: i64,
    partition_leader_epoch
}

struct PartitionData<'a, T> {
    index: i32,
    records: &'a [T],
}

struct TopicData<'a, T> {
    name: &'a str,
    partition_data: PartitionData<'a, T>,
}

pub(crate) struct ProduceMessage<'a, T> {
    transaction_id: Option<i32>,
    acks: Acks,
    timeout_ms: i32,
    topic_data: TopicData<'a, T>,
}

/// This enum identifies the expected partition schema for 
/// computing the partition for a given piece of data.
///
pub enum PartitionSchema {
    /// Single means that this producer should send all data to a single partition. Use this if you
    /// have your own custom schema and a producer per partition.
    Single(i32),

    /// RoundRobin means that this producer will rotate data between all partitions in one at a
    /// time. This is a good way to ensure that data is distributed evenly across brokers.
    RoundRobin,

    /// Hashes the key and uses the hash to compute the partition.
    Hash,
}

pub struct Producer<'a> {
    client: Client,
    topic: &'a str,
    partition_schema: PartitionSchema,
}

impl<'a> Producer<'a> {
    pub fn new(bootstrap_servers: Vec<&'static str>, topic: &'a str) -> Result<Self, &'static str> {
        let client = bootstrap_servers
            .iter()
            .find_map(|url| Client::new(url).ok())
            .unwrap();
        Ok(Self { client, topic, partition_schema: PartitionSchema::RoundRobin })
    }
}

use crate::errors::KafkaError;
use serde::Serialize;

impl<'a> Producer<'a> {
    pub fn produce<T: Serialize>(&self, items: &Vec<T>) -> Result<ResponseMessage, KafkaError> {
        let message = ProduceMessage {
            transaction_id: None,
            acks: Acks::Leader,
            timeout_ms: 1000i32,
            topic_data: TopicData {
                name: self.topic,
                partition_data: self.partition_data_for_items(&items)
            }.
        };
        self.client.send(ApiKey::Produce, message)
    }

    fn partition_data_for_items<T: Serialize>(&self, items: &[T]) -> {

    }
}
