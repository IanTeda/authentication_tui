//-- ./src/handlers/keys.rs

#![allow(unused)] // For beginning only.

//! What to do with key events
//! ---

use crate::{client, state};
use crate::{domain, prelude::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct KeyEventHandler {
    pub config: crate::Config,
}

impl KeyEventHandler {
    /// Construct a new key event handler
    pub fn new(config: crate::Config) -> Self {
        Self { config }
    }

    /// Handles the key events and updates the state of [`App`].
    pub async fn handle_event(
        &self,
        key_event: KeyEvent,
        state: &mut state::State,
    ) -> Result<()> {
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
            KeyCode::Char('r') => {
                // state.backend.status = domain::BackendStatus::Online;
                let address = self.config.backend.address();

                let mut rpc_client = client::RpcClient::new(address).await?;
                let request_message = tonic::Request::new(client::rpc::Empty {});
                let response = rpc_client.utilities().ping(request_message).await?;
                let (response_metadata, _response_message, _response_extensions) =
                    response.into_parts();

                let grpc_status = response_metadata.get::<&str>("grpc-status");

                if let Some(status) = grpc_status {
                    if status == "0" {
                        state.backend.status = domain::BackendStatus::Online;
                        let toast_message = format!("Backend server is: {:?}", state.backend.status);
                        let toast = domain::Toast::new(toast_message);
                        state.toast.queue.push_back(toast);
                    }
                }
            }
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
