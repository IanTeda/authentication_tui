
#[derive(Clone)]
pub struct RpcClient {
    utilities: UtilitiesClient,
}

/// Convenience type alias for authentication client.
pub type UtilitiesClient =
    rpc::utilities_client::UtilitiesClient<transport::Channel>;

impl RpcClient {
    /// Returns the utilities client.
    pub fn utilities(&mut self) -> &mut UtilitiesClient {
        &mut self.utilities
    }

    /// Spawn a new tonic client based on the tonic server
    pub async fn new(
        address: String,
    ) -> Result<Self, TuiError> {
        // Build Tonic Client channel
        let uri: tonic::transport::Uri = address.parse()?;
        let endpoint = transport::Channel::builder(uri);
        let channel: transport::Channel = endpoint.connect().await?;

        let utilities = UtilitiesClient::new(channel.clone());

        let client = RpcClient { utilities };

        Ok(client)
    }
}