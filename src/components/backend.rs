//-- ./src/ui/container.rs

#![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use crossterm::event as crossterm;
use ratatui::prelude::*;
use std::{net, time};

use crate::{
    client::{rpc, RpcClient},
    components, domain,
    prelude::*,
    Config,
};

pub struct BackendComponent {
    /// Backend socket address
    address: net::SocketAddr,

    /// Access token returned during login
    access_token: Option<String>,

    /// When was the access token received
    access_token_time: Option<time::Instant>,

    /// Refresh token (session) return by the backend
    refresh_token: Option<String>,

    /// When was the refresh token received
    refresh_token_time: Option<time::Instant>,

    /// Is the backend online
    status: domain::BackendStatus,

    /// When was the backend last checked for being online
    status_checked_time: Option<time::Instant>,
}

impl Default for BackendComponent {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let localhost = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
        let port = 8081;
        let address = net::SocketAddr::new(localhost, port);
        let access_token = None;
        let access_token_time = None;
        let refresh_token = None;
        let refresh_token_time = None;
        let status = domain::BackendStatus::Offline;
        let status_checked_time = None;

        Self {
            address,
            access_token,
            access_token_time,
            refresh_token,
            refresh_token_time,
            status,
            status_checked_time,
        }
    }
}

impl BackendComponent {
    /// Construct a new backend component
    pub fn new(address: net::SocketAddr) -> Self {
        Self {
            address,
            ..Default::default()
        }
    }

    /// Check if the backend authentication server is online
    async fn check_online(&self) -> Result<String> {
        let mut client = RpcClient::new(self.address).await?;

        let request_message = tonic::Request::new(rpc::Empty {});

        let response = client.utilities().ping(request_message).await?;

        let (_response_metadata, response_message, _response_extensions) =
            response.into_parts();


        // println!("Message: {:?}", response_message);

        Ok(response_message.message)
    }
}

impl components::Component for BackendComponent {
    fn handle_key_event(
        &mut self,
        key_event: crossterm::KeyEvent,
    ) -> Result<Option<domain::Action>> {
        let action = match key_event.code {
            crossterm::KeyCode::Char('r') => {
                // Return action for update
                domain::Action::BackendStatusUpdate
            }
            _ => domain::Action::Nil,
        };

        Ok(Some(action))
    }

    fn update(&mut self, action: domain::Action) -> Result<Option<domain::Action>> {
        let update_option = match action {
            domain::Action::Tick => {
                // add any logic here that should run on every tick
                None
            }
            domain::Action::Render => {
                // add any logic here that should run on every render
                None
            }
            domain::Action::BackendStatus(status) => {
                // Build toast message
                let message =
                    format!("Backend authentication server is now: {}", status);

                // Build toast instance
                let toast = domain::Toast::new(message)
                    .kind(domain::ToastKind::Notification);

                // Return action for update
                let action = domain::Action::Toast(toast);

                Some(action)
            }
            domain::Action::BackendStatusUpdate => {
                // Check backend status
                // self.check_online();
                self.status = domain::BackendStatus::Online;
                let status = domain::BackendStatus::Online;
                let action = domain::Action::BackendStatus(status);

                // let message = self.check_online().await?;

                Some(action)
            }
            _ => None,
        };

        // Ok(None)
        Ok(update_option)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let _ = area;
        let _ = frame;
        Ok(())
    }
}
