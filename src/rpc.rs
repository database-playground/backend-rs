pub mod dbrunner {
    tonic::include_proto!("dbrunner.v1");
}

use dbrunner::db_runner_service_client::DbRunnerServiceClient;
use ecow::EcoString;

pub type DbRunnerClient = DbRunnerServiceClient<tonic::transport::Channel>;

pub async fn dbrunner_client() -> Result<DbRunnerClient, Error> {
    let addr = std::env::var("DBRUNNER_ADDR").map_err(|_| Error::NoClientAddress {
        client: "DBRUNNER_ADDR".into(),
    })?;
    let client = DbRunnerServiceClient::connect(addr).await?;

    return Ok(client);
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no {client} address specified in environment")]
    NoClientAddress { client: EcoString },

    #[error(transparent)]
    TransportError(#[from] tonic::transport::Error),
}
