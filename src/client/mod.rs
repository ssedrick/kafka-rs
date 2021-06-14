use crate::errors::KafkaError;
use crate::serde::{
    request::{RequestHeader, RequestMessage},
    response::ResponseMessage,
    ApiKey,
};
use serde::Serialize;

#[cfg(feature = "tokio-rt")]
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpSocket,
};

struct ApiVersion {
    api_key: ApiKey,
    min_version: i16,
    max_version: i16,
}

type ApiVersions = Vec<ApiVersion>;

pub struct Client {
    bootstrap_server: &'static str,
    api_versions: ApiVersions,

    #[cfg(not(any(feature = "tokio-rt")))]
    socket: std::net::TcpStream,

    #[cfg(feature = "tokio-rt")]
    socket: tokio::net::TcpSocket,
}

#[cfg(not(feature = "tokio-rt"))]
impl Client {
    pub fn new(bootstrap_server: &'static str) -> std::io::Result<Self> {
        Ok(Self {
            bootstrap_server: bootstrap_server,
            api_versions: Vec::new(),
            socket: std::net::TcpStream::connect(bootstrap_server)?,
        })
    }

    pub fn build(&self) -> Result<(), KafkaError> {
        self.bootstrap()?;
        self.get_api_versions()?;
        Ok(())
    }

    fn bootstrap(&self) -> Result<(), KafkaError> {
        self.get_metadata()
    }

    fn get_metadata(&self) -> Result<(), KafkaError> {
        Ok(())
    }

    fn get_api_versions(&self) -> Result<(), KafkaError> {
        Ok(())
    }

    fn get_latest_version(&self, api_key: ApiKey) -> Result<i16, KafkaError> {
        let api_version = self.api_versions.iter().find(|api| api.api_key == api_key);
        match api_version {
            Some(api_version) => Ok(api_version.max_version),
            None => Err(KafkaError::UnknownServerError),
        }
    }

    pub fn send<T: Serialize>(
        &self,
        api: ApiKey,
        message: T,
    ) -> Result<ResponseMessage, KafkaError> {
        let version = self.get_latest_version(api)?;
        let request = RequestMessage {
            header: RequestHeader {
                api_key: api,
                api_version: version,
                correlation_id: 13i32,
                client_id: None,
            },
            body: message,
        };
        // TCP send
        Ok(ResponseMessage::new(13i32 /*, buffer */))
    }
}

#[cfg(feature = "tokio-rt")]
impl Client {
    pub fn new(bootstrap_server: &'static str) -> std::io::Result<Self> {
        Ok(Self {
            bootstrap_server: bootstrap_server,
            api_versions: Vec::new(),
            socket: std::net::TcpStream::connect(bootstrap_server)?,
        })
    }

    pub async fn new(bootstrap_server: &'static str) -> tokio::io::Result<Self> {
        let stream = tokio::net::TcpStream::new_v4()?;
        let socket = stream::connect(bootstrap_server).await?;

        Ok(Self {
            bootstrap_server: bootstrap_server,
            api_version: Vec::new(),
            socket: socket,
        })
    }

    pub async fn build(&self) -> tokio::io::Result<()> {
        self.bootstrap().await?;
        self.get_api_versions().await?;
        Ok(())
    }

    pub async fn bootstrap(&self) -> tokio::io::Result<()> {
        Ok(())
    }

    async fn get_metadata(&self) -> tokio::io::Result<(), KafkaError> {
        Ok(())
    }

    async fn get_api_versions(&self) -> tokio::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn it_builds() {
        let client = Client::new("127.0.0.1:9092").unwrap();
        let res = client.build();
        assert!(res.is_ok());
    }
}
