//-- ./src/handlers/keys.rs

#![allow(unused)] // For beginning only.

//! What to do with key events
//! ---

use crate::{client, services, state};
use crate::{domain, prelude::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc;

pub struct KeyEventHandler {
    pub config: crate::Config,
    action_tx: Option<mpsc::UnboundedSender<domain::Action>>,
}

impl KeyEventHandler {
    /// Construct a new key event handler
    pub fn new(config: crate::Config) -> Self {
        let action_tx = None;
        Self { config, action_tx }
    }

    /// Handles the key events and updates the state of [`App`].
    pub async fn handle_event(
        &self,
        key_event: KeyEvent,
        state: &mut state::State,
    ) -> Result<()> {
        // Assign socket address for communicating with the backend
        let rpc_server_address = self.config.backend.address();

        // Build the rpc client, setting Offline if error returned
        let rpc_client: Option<client::RpcClient> =
            match client::RpcClient::new(rpc_server_address).await {
                // Match call returned an ok result
                Ok(rpc_client) => Some(rpc_client),

                // Match call returned an error result
                Err(error) => {
                    // Set state to Offline
                    state.backend.status = domain::BackendStatus::Offline;

                    // Provide toast of status to user
                    let toast_message =
                        format!("Backend server is: {:?}", state.backend.status);
                    let toast = domain::Toast::new(toast_message);
                    state.toast.queue.push_back(toast);

                    // Send error to tracing log
                    tracing::error!("Error connecting to backend server: {}", error);

                    // Return None
                    None
                }
            };

        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Char('q') => {
                state.app.is_running = false;
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    state.app.is_running = false;
                }
            }
            // KeyCode::Char('t') => {
            //     let toast_message = domain::Toast::new("Added toast to queue").kind(domain::ToastKind::Error);
            //     state.toast.queue.push_back(toast_message);
            // }

            // Refresh backend server status
            KeyCode::Char('r') => {
                if let Some(rpc_client) = rpc_client {
                    // Construct a utilities service
                    let mut utilities_service =
                        services::UtilitiesService::new(rpc_client);

                    // Check if backend is online
                    if utilities_service.is_online().await {
                        // Set backend status to Online
                        state.backend.status = domain::BackendStatus::Online;

                        // Provide toast of status to user
                        toast_backend_status(state);

                    // Else false
                    } else {
                        // Set state to Offline
                        state.backend.status = domain::BackendStatus::Offline;

                        // Provide toast of status to user
                        toast_backend_status(state);
                    }
                } else {
                    // Set state to Offline
                    state.backend.status = domain::BackendStatus::Offline;

                    // Provide toast of status to user
                    toast_backend_status(state);
                }
            }

            // Escape from the tui application
            KeyCode::Esc => {
                state.toast.current = None;
            }
            // // Counter handlers
            // KeyCode::Right => {
            //     state.increment_counter();
            // }
            // KeyCode::Left => {
            //     state.decrement_counter();
            // }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }
}

fn toast_backend_status(state: &mut state::State) {
    let toast_message = format!("Backend server is: {:?}", state.backend.status);
    let toast = domain::Toast::new(toast_message);
    state.toast.queue.push_back(toast);
}

// /// Handles the key events and updates the state of [`App`].
// pub fn handle_event(key_event: KeyEvent, state: &mut state::State) -> Result<()> {
//     match key_event.code {
//         // Exit application on `ESC` or `q`
//         KeyCode::Char('q') => {
//             state.app.is_running = false;
//         }
//         // Exit application on `Ctrl-C`
//         KeyCode::Char('c') => {
//             if key_event.modifiers == KeyModifiers::CONTROL {
//                 state.app.is_running = false;
//             }
//         }
//         // KeyCode::Char('t') => {
//         //     let toast_message = domain::Toast::new("Added toast to queue").kind(domain::ToastKind::Error);
//         //     state.toast.queue.push_back(toast_message);
//         // }
//         KeyCode::Char('r') => {
//             // state.backend.status = domain::BackendStatus::Online;
//         }
//         KeyCode::Esc => {
//             state.toast.current = None;
//         }
//         // // Counter handlers
//         // KeyCode::Right => {
//         //     state.increment_counter();
//         // }
//         // KeyCode::Left => {
//         //     state.decrement_counter();
//         // }
//         // Other handlers you could add here.
//         _ => {}
//     }
//     Ok(())
// }
