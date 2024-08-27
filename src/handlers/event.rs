//-- ./src/handlers/crossterm.rs

// #![allow(unused)] // For beginning only.

//! Module for mapping crossterm terminal backend events to tui application events
//! 
//! ---

use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

// use crate::prelude::*;

/// TUI Application events.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Closed,
    Error,
    FocusGained,
    FocusLost,
    Init,
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Tick,
}

#[derive(Debug)]
pub struct EventLoopHandler {
    /// How many times per second should a tick loop (event) happen
    tick_rate: f64,

    /// How many times per second should the UI be rendered
    frame_rate: f64,

    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,

    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,

    /// Event handler thread.
    task: tokio::task::JoinHandle<()>,

    /// Signal the cancellation of the event loop in async
    cancellation_token: tokio_util::sync::CancellationToken,
}

impl EventLoopHandler {
    /// Construct a new instance of the Crossterm event loop handler
    pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
        // Initiate send receive event channels
        let (sender, receiver) = mpsc::unbounded_channel();

        let handler = tokio::spawn(async move {});

        let cancellation_token = CancellationToken::new();

        Self {
            tick_rate,
            frame_rate,
            sender,
            receiver,
            task: handler,
            cancellation_token,
        }
    }

    /// Cancel the existing token
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Start the event loop
    pub fn start(&mut self) {
        // Cancel any existing event tasks
        self.cancel();

        // Generate new cancellation token
        self.cancellation_token = CancellationToken::new();

        // Construct crossterm event loop handler
        let event_loop = Self::event_loop(
            self.sender.clone(),
            self.cancellation_token.clone(),
            self.tick_rate,
            self.frame_rate,
        );

        // Move task into an async thread
        self.task = tokio::spawn(async {
            event_loop.await;
        })
    }

    /// The event loop
    async fn event_loop(
        sender: mpsc::UnboundedSender<Event>,
        cancellation_token: tokio_util::sync::CancellationToken,
        tick_rate: f64,
        frame_rate: f64,
    ) {
        let mut event_stream = crossterm::event::EventStream::new();

        // Calculate tick per second
        let mut tick_interval = tokio::time::interval(
            std::time::Duration::from_secs_f64(1.0 / tick_rate),
        );

        // Calculate frame rate per second
        let mut frame_interval = tokio::time::interval(
            std::time::Duration::from_secs_f64(1.0 / frame_rate),
        );

        // Check future channels are working. If this fails, then it's likely a
        // bug in the calling code
        sender.send(Event::Init).expect("failed to send init event");

        loop {
            // Tokio’s select! macro allows us to wait on multiple async
            // computations and returns when a single computation completes.
            let event = tokio::select! {
                // If the cancellation token is canceled break out of the loop
                _ = cancellation_token.cancelled() => break,

                // On the tick interval add a tick event to the que
                _ = tick_interval.tick() => Event::Tick,

                // on the frame interval add a render event to the que
                _ = frame_interval.tick() => Event::Render,

                // Poll the crossterm backend terminal stream for next crossterm terminal
                // event and send matched event to the que
                crossterm_event = event_stream.next().fuse() => match crossterm_event {
                    Some(Ok(event)) => match event {
                        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
                        CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
                        CrosstermEvent::Resize(x, y) => Event::Resize(x, y),
                        CrosstermEvent::FocusLost => Event::FocusLost,
                        CrosstermEvent::FocusGained => Event::FocusGained,
                        CrosstermEvent::Paste(s) => Event::Paste(s),
                        _ => continue, // ignore other events
                    }
                    Some(Err(_)) => Event::Error,
                    None => break, // the event stream has stopped and will not produce any more events
                },
            };

            // Send event to the que, if there is an error break out from the loop
            if sender.send(event).is_err() {
                // the receiver has been dropped, so there's no point in continuing the loop
                break;
            }
        }
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}