//-- ./src/services/utilities.rs

// #![allow(unused)] // For beginning only.

use crate::client;

/// Utilities service instance
pub struct UtilitiesService {
    // RPC Client
    pub rpc_client: client::RpcClient,
}

impl UtilitiesService {
    /// Construct a new utilities service using the defined rpc client
    pub fn new(rpc_client: client::RpcClient) -> Self {
        Self { rpc_client }
    }

    /// Check if the backend authentication server is online, returning true if
    /// it is.
    pub async fn is_online(&mut self) -> bool {
        // Build the rpc request message
        let request_message = tonic::Request::new(client::rpc::Empty {});

        // Send prc request
        let response = match self.rpc_client.utilities().ping(request_message).await
        {
            Ok(response) => response,
            Err(_) => return false,
        };

        let (response_metadata, _response_message, _response_extensions) =
            response.into_parts();

        let grpc_status = response_metadata.get::<&str>("grpc-status");

        if let Some(status) = grpc_status {
            status == "0"
        } else {
            false
        }
    }
}
