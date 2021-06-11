use crate::errors::KafkaError;
use crate::serde::ApiKey;

#[cfg(feature = "async-tokio")]
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpSocket,
};

struct ApiVersion {
    api_key: ApiKey,
}

type ApiVersions = Vec<ApiVersion>;

struct Client {
    bootstrap_server: &'static str,
    api_versions: ApiVersions,

    #[cfg(feature = "async-tokio")]
    read: Option<AsyncReadExt>,
    #[cfg(feature = "async-tokio")]
    write: Option<TokioWriteExt>,

    read: Option<std::io::BufReader<u8>>,
    write: Option<std::io::BufWriter<std::net::TcpStream>>,
}

impl Client {
    pub fn new(bootstrap_server: &'static str) -> Self {
        Self {
            bootstrap_server: bootstrap_server,
            api_versions: Vec::new(),
            read: None,
            write: None,
        }
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
        #[cfg(feature = "async-tokio")]
        self.get_metadata_tokio()?
    }

    fn get_metadata_tokio(&self) -> Result<(), KafkaError> {
        self.read = Some(tokio::net::TcpStream::new_v4());
    }

    fn get_api_versions(&self) -> Result<(), KafkaError> {
        Ok(())
    }
}
