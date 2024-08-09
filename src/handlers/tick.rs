//-- ./src/handlers/tick.rs

// #![allow(unused)] // For beginning only.

//! Handles the tick event and update the application as needed
//! ---

use std::time;

use crate::{states, TuiResult};

/// Handles the key events and updates the state of [`App`].
pub async fn tick(state: &mut states::App) -> TuiResult<()> {

    if state.toast.show {
        let three_seconds = time::Duration::from_secs(3);
        if state.toast.displayed_on.elapsed() >= three_seconds {
            state.toast.show = !state.toast.show
        }
    };

    Ok(())
}