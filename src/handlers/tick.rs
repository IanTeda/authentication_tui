//-- ./src/handlers/tick.rs

// #![allow(unused)] // For beginning only.

//! Handles the tick event and update the application as needed
//! ---

use std::time;

use crate::{rpc, states, RpcClient, TuiResult};

/// Handles the key events and updates the state of [`App`].
pub async fn tick(state: &mut states::App) -> TuiResult<()> {
    let now = time::Instant::now();

    // If there is toast showing hide it after three seconds
    if state.toast.show {
        let three_seconds = time::Duration::from_secs(3);
        if state.toast.displayed_on.elapsed() >= three_seconds {
            state.toast.show = !state.toast.show
        }
    };

    match state.backend.status_checked_on {
        None => {
            let address = state.backend.address;
            let mut rpc_client = RpcClient::new(address).await?;
            let request_message = tonic::Request::new( rpc::Empty {});
            let response = rpc_client.utilities().ping(request_message).await?;
            
            let (_response_metadata, _response_message, _response_extensions) =
                response.into_parts();

            state.backend.is_online = true;
            state.backend.status_checked_on = Some(now);
            state.toast.message = "Backend server is online".to_string();
            state.toast.show = true;
        },
        Some(_) => {
            // let status_check_duration = time::Duration::from_secs(60*10);

        },
    }

    Ok(())
}