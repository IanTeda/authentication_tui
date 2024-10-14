//-- ./src/handlers/tick.rs

#![allow(unused)] // For beginning only.

//! Application tick events
//!
//! The frequency of this event is set in config and triggered in CrosstermEventsHandler
//! ---

// use std::{os::unix::net, time};
use std::{net, time};
use tokio::sync::mpsc;

use crate::{client, domain, services, state};

const TOAST_DURATION: time::Duration = time::Duration::from_secs(3);
const STATUS_CHECK_DURATION: time::Duration = time::Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct TickEventHandler {
    /// Action sender
    action_sender: mpsc::UnboundedSender<domain::Action>,

    /// Time since last tick
    last_tick_update: time::Instant,

    /// The number of ticks
    tick_count: u32,

    /// Rolling ticks per second calculation
    ticks_per_second: f64,
}

impl TickEventHandler {
    /// New TickEventHandler instance
    pub fn init(action_sender: mpsc::UnboundedSender<domain::Action>) -> Self {
        let last_tick_update = time::Instant::now();
        let tick_count = 0;
        let ticks_per_second = 0.0;
        Self {
            action_sender,
            last_tick_update,
            tick_count,
            ticks_per_second,
        }
    }

    /// What to do each tick event cycle
    pub async fn handle_event(&mut self, state: &mut state::State) {
        //-- 1. Calculate tick statistics
        // Calculate the current tick rate, based on the last tick time
        self.calculate_tick_rate();

        // Update the application tick rate state
        state.app.ticks_per_second = self.ticks_per_second;

        // // If the status has been checked previously (not None), check if status
        // // check duration has elapsed
        // if let Some(checked_on) = state.backend.status_checked_on {
        //     if checked_on.elapsed() > STATUS_CHECK_DURATION {
        //         state.backend.status = domain::BackendStatus::Offline;
        //         state.backend.status_checked_on = Some(time::Instant::now());
        //     }
        // // Else check the backend authentication server status
        // } else {
        //     state.backend.status = domain::BackendStatus::Offline;
        //     state.backend.status_checked_on = Some(time::Instant::now());
        // }

        // // If the status_checked_on option has a value process
        // if let Some(checked) = state.backend.status_checked_on {
        //     // Assign socket address for communicating with the backend
        //     let rpc_server_address = self.config.backend.address();

        //     // Build the rpc client, setting Offline if error returned
        //     let rpc_client: Option<client::RpcClient> =
        //         match client::RpcClient::new(rpc_server_address).await {
        //             // Match call returned an ok result
        //             Ok(rpc_client) => Some(rpc_client),

        //             // Match call returned an error result
        //             Err(error) => {
        //                 // Set state to Offline
        //                 state.backend.status = domain::BackendStatus::Offline;

        //                 // Provide toast of status to user
        //                 let toast_message =
        //                     format!("Backend server is: {:?}", state.backend.status);
        //                 let toast = domain::Toast::new(toast_message);
        //                 state.toast.queue.push_back(toast);

        //                 // Send error to tracing log
        //                 tracing::error!("Error connecting to backend server: {}", error);

        //                 // Return None
        //                 None
        //             }
        //         };

        //     // If the rpc_client option has a value process
        //     if let Some(rpc_client) = rpc_client {
        //         // If checked elapsed time is greater check
        //         if checked.elapsed() > STATUS_CHECK_DURATION {
        //             // Construct a utilities service
        //             let mut utilities_service =
        //                 services::UtilitiesService::new(rpc_client);

        //             // Check if backend is online
        //             if utilities_service.is_online().await {
        //                 // Set backend status to Online
        //                 state.backend.status = domain::BackendStatus::Online;

        //                 // Provide toast of status to user
        //                 toast_backend_status(state);

        //             // Else false
        //             } else {
        //                 // Set state to Offline
        //                 state.backend.status = domain::BackendStatus::Offline;

        //                 // Provide toast of status to user
        //                 toast_backend_status(state);
        //             }
        //         }
        //     }
        // }

        // //-- 2. Manage toast messages
        // // If we have an optional toast message wait for elapsed time to exceed
        // if let Some(ref mut t) = state.toast.current {
        //     // If toast duration is exceeded set option to None, to display
        //     // the next toast message in the queue.
        //     if t.shown_at.elapsed() > TOAST_DURATION {
        //         state.toast.current = None;
        //     }
        // // Else if we have None optional toast and there is something in
        // // the toast queue pop it into the optional toast
        // } else if let Some(mut t) = state.toast.queue.pop_front() {
        //     t.shown_at = time::Instant::now();
        //     state.toast.current = Some(t);
        // }
    }

    /// Update tick rate information
    fn calculate_tick_rate(&mut self) {
        // Increment the tick count by one
        self.tick_count += 1;

        // Represent now as an Instance
        let now = time::Instant::now();

        // Calculate elapsed time since last tick
        let elapsed = (now - self.last_tick_update).as_secs_f64();

        // If elapsed time is grater than one second update tick state
        if elapsed >= 1.0 {
            self.ticks_per_second = self.tick_count as f64 / elapsed;
            self.last_tick_update = now;
            self.tick_count = 0;
        }
    }

    /// Return the current application tick rate per second
    pub fn tick_rate(self) -> f64 {
        self.ticks_per_second
    }
}

fn toast_backend_status(state: &mut state::State) {
    let toast_message = format!("Backend server is: {:?}", state.backend.status);
    let toast = domain::Toast::new(toast_message);
    state.toast.queue.push_back(toast);
}

async fn get_rpc_client(
    rpc_server_address: net::SocketAddr,
) -> Option<client::RpcClient> {
    match client::RpcClient::new(rpc_server_address).await {
        // Match call returned an ok result
        Ok(rpc_client) => Some(rpc_client),

        // Match call returned an error result
        Err(error) => None,
    }
}
