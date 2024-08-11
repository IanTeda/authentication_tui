//-- ./src/proto.rs

// #![allow(unused)] // For beginning only.

//! Modules for access the backend RPC server
//!
//! * [Tonic LND client](https://github.com/Kixunil/tonic_lnd/blob/master/src/lib.rs)

use crate::TuiError;
use std::net;
use tonic::transport;

/// Bring protobuf generated code into scope
pub mod rpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("authentication");
}

/// Convenience type alias for authentication client.
pub type UtilitiesClient =
    rpc::utilities_client::UtilitiesClient<transport::Channel>;

/// Tonic Client
#[derive(Clone)]
pub struct RpcClient {
    utilities: UtilitiesClient,
}

impl RpcClient {
    /// Returns the utilities client.
    pub fn utilities(&mut self) -> &mut UtilitiesClient {
        &mut self.utilities
    }

    /// Create an new tonic client for the given Socket Address
    pub async fn new(
        address: net::SocketAddr,
    ) -> Result<Self, TuiError> {
        // Build Tonic Client channel
        let address = format!("http://{}", address);
        let uri: tonic::transport::Uri = address.parse()?;
        let channel = transport::Channel::builder(uri).connect().await?;

        // Build the endpoint channels
        let utilities = UtilitiesClient::new(channel.clone());

        // build the RPC Client
        let client = RpcClient { utilities };

        Ok(client)
    }
}
