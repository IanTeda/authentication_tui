//-- ./src/controllers/update_backend_status.rs

//! Try to connect to the backend status and ping for a pong.
//! 
//! A connection error will set the the status to Offline
//! 
//! A Ok response will set the status to Online

use std::time;

use crate::{client, domain, services};

impl crate::App {
    /// Try to connect to the backend status and ping for a pong.
    /// 
    /// A connection error will set the the status to Offline
    /// 
    /// A Ok response will set the status to Online
    pub async fn update_backend_status(&mut self) {
        // Set the status checked on time instance to now
        self.state.backend.status_checked_on = Some(time::Instant::now());

        // Assign socket address for communicating with the backend
        let rpc_server_address = self.config.backend.address();

        // Build the rpc client, setting Offline if error returned
        let rpc_client: Option<client::RpcClient> =
            match client::RpcClient::new(rpc_server_address).await {
                // Match call returned an ok result
                Ok(rpc_client) => Some(rpc_client),

                // Match call returned an error result
                Err(error) => {
                    // Set state to Offline on error
                    self.state.backend.status = domain::BackendStatus::Offline;

                    // Send error to tracing log
                    tracing::error!("Error connecting to backend server: {}", error);

                    // Return None
                    None
                }
            };

        // If the rpc_client option has a value process
        if let Some(rpc_client) = rpc_client {
            // Construct a utilities service
            let mut utilities_service = services::UtilitiesService::new(rpc_client);

            // Check if backend is online
            if utilities_service.is_online().await {
                // Set backend status to Online
                self.state.backend.status = domain::BackendStatus::Online;

            // Else false
            } else {
                // Set state to Offline
                self.state.backend.status = domain::BackendStatus::Offline;
            }
        }
    }
}