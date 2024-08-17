//-- ./src/event.rs

#![allow(unused)] // For development only

//! Handles the terminal events (key press, mouse click, resize, etc.)
//! ---
use core::time;
use std::time::Duration;
use serde;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::{TuiError, TuiResult};

/// Terminal events.
// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize )]
#[derive(Clone, Debug)]
pub enum Event {
    Init,
    
    Quit,
    
    Error,

    Closed,

    Render,

    /// Terminal tick.
    Tick,

    Frame,

    FocusGained,

    FocusLost,

    /// Paste
    Paste(String),

    /// Key press.
    Key(KeyEvent),

    /// Mouse click/scroll.
    Mouse(MouseEvent),

    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
// #[allow(dead_code)]
#[derive(Debug)]
pub struct CrosstermEventHandler {
    /// Event sender channel.
    event_sender: mpsc::UnboundedSender<Event>,

    /// Event receiver channel.
    event_receiver: mpsc::UnboundedReceiver<Event>,

    /// Event handler thread.
    task: tokio::task::JoinHandle<()>,

    /// Tokio channel cancellation token
    cancellation_token: tokio_util::sync::CancellationToken,
}

impl CrosstermEventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64, frame_rate: u64) -> Self {
        // Set the timing of the loop
        let tick_rate = time::Duration::from_millis(tick_rate);
        let frame_rate = time::Duration::from_millis(frame_rate);

        // Set tokio async channels
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();
        let _event_sender = event_sender.clone();

        // Use token to stop tokio tasks on request
        let cancellation_token = tokio_util::sync::CancellationToken::new();
        let _cancellation_token = cancellation_token.clone();

        // The tokio async application loop
        let task = tokio::spawn(async move {
            // Read terminal backend events
            let mut event_stream = crossterm::event::EventStream::new();

            // Set the loop tick interval
            let mut tick_interval = tokio::time::interval(tick_rate);
            let mut frame_interval = tokio::time::interval(frame_rate);

            loop {
                // Set the loop tick delay
                let tick_delay = tick_interval.tick();

                // Set the frame rate tick delay
                let frame_delay = frame_interval.tick();

                // Get the next backend terminal event
                let crossterm_stream = event_stream.next().fuse();

                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        break;
                    }
                    _ = tick_delay => {
                        _event_sender.send(Event::Tick).unwrap();
                    }
                    _ = frame_delay => {
                        _event_sender.send(Event::Frame).unwrap();
                    }
                    crossterm_event = crossterm_stream => match crossterm_event {
                        Some(Ok(evt)) => match evt {
                            CrosstermEvent::Key(key) => {
                                // If statement avoids window double key press bug
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _event_sender.send(Event::Key(key)).unwrap();
                                }
                            },
                            CrosstermEvent::Mouse(mouse) => {
                                _event_sender.send(Event::Mouse(mouse)).unwrap();
                            },
                            CrosstermEvent::Resize(x, y) => {
                                _event_sender.send(Event::Resize(x, y)).unwrap();
                            },
                            CrosstermEvent::FocusLost => {
                                _event_sender.send(Event::FocusLost).unwrap();
                            },
                            CrosstermEvent::FocusGained => {
                                _event_sender.send(Event::FocusGained).unwrap();
                            },
                            CrosstermEvent::Paste(s) => {
                                _event_sender.send(Event::Paste(s)).unwrap();
                            },
                        },
                        Some(Err(_)) => {
                            _event_sender.send(Event::Error).unwrap();
                        },
                        // the event stream has stopped and will not produce any more events, so break.
                        None => break,
                    }
                };
            }
        });
        Self {
            event_sender,
            event_receiver,
            task,
            cancellation_token,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> TuiResult<Event> {
        self.event_receiver
            .recv()
            .await
            .ok_or(TuiError::Static("There is an IO Error"))
    }


  pub async fn stop(&mut self) -> TuiResult<()> {
    self.cancellation_token.cancel();
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(1));
      counter += 1;
      if counter > 50 {
        self.task.abort();
      }
      if counter > 100 {
        tracing::error!("Failed to abort task in 100 milliseconds for unknown reason");
        break;
      }
    }
    Ok(())
  }
}
