#![allow(unused)] // For development only

use tokio::sync::mpsc;

use crate::{states, TuiResult};


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::Display)]
pub enum Action {
    // Check the backend server is online
    BackendOnline,

    // Application error
    Error(String),

    // Show the help pop up
    // Help,

    // Do thing this loop
    Nil,

    // Quit the TUI
    Quit,

    // Every tick
    Tick,

    Render,

    Resize(u16, u16),
}

pub struct ActionHandler{
    

    /// Event sender channel.
    action_sender: mpsc::UnboundedSender<Action>,

    /// Event receiver channel.
    action_receiver: mpsc::UnboundedReceiver<Action>,
}

// impl ActionHandler {
//     /// Constructs a new instance of [`ActionHandler`].
//     pub fn new(&mut state: states::AppState) -> TuiResult<Self> {


//         unimplemented!()
//     }
// }
