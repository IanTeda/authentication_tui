//-- ./src/handler.rs

//! Handles the key press events and updates the application
//! ---
use crate::{app::{App, AppResult}, rpc};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('p') => {
            panic!("Intentional panic");
        }
        //
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if key_event.modifiers == KeyModifiers::CONTROL {

                // let address = "http://127.0.0.1:8091".to_string();

                let address = format!(
                "http://{}:{}",
                    app.config.backend.ip,
                    app.config.backend.port,
                  );

                let mut rpc_client = crate::RpcClient::new(address).await?;
                let request_message = tonic::Request::new( rpc::Empty {});
                let response = rpc_client.utilities().ping(request_message).await?;
                let (_response_metadata, _response_message, _response_extensions) =
                    response.into_parts();

                app.popup.title = String::from("This is a test");
                app.popup.message = format!("Message: {:?}", _response_metadata.clone());
                app.popup.show = !app.popup.show;
                app.backend_is_online = true;
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
