//-- ./src/handlers/crossterm.rs

// #![allow(unused)] // For beginning only.

//! Module for mapping crossterm terminal backend events to tui application events
//!
//! ---

use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{domain, prelude::*};

// use crate::prelude::*;

#[derive(Debug)]
pub struct CrosstermEventsHandler {
    /// How many times per second should a tick loop (event) happen
    tick_rate: f64,

    /// How many times per second should the UI be rendered
    frame_rate: f64,

    /// Event sender channel.
    sender: mpsc::UnboundedSender<domain::Event>,

    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<domain::Event>,

    /// Event handler thread.
    task: tokio::task::JoinHandle<()>,

    /// Signal the cancellation of the event loop in async
    cancellation_token: tokio_util::sync::CancellationToken,
}

impl CrosstermEventsHandler {
    /// Construct a new instance of the Crossterm event loop handler
    pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
        // Initiate send receive event channels
        let (sender, receiver) = mpsc::unbounded_channel();

        // Construct the task handle for the channel
        let task = tokio::spawn(async move {});

        // Generate a new cancellation token for breaking out of the event loop
        let cancellation_token = CancellationToken::new();

        Self {
            tick_rate,
            frame_rate,
            sender,
            receiver,
            task,
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
        sender: mpsc::UnboundedSender<domain::Event>,
        cancellation_token: tokio_util::sync::CancellationToken,
        tick_rate: f64,
        frame_rate: f64,
    ) {
        let mut crossterm_event_stream = crossterm::event::EventStream::new();

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
        sender
            .send(domain::Event::Init)
            .expect("failed to send init event");

        loop {
            // Tokio’s select! macro allows us to wait on multiple async
            // computations and returns when a single computation completes.
            let event = tokio::select! {
                // If the cancellation token is canceled break out of the loop
                _ = cancellation_token.cancelled() => break,

                // On the tick interval add a tick event to the que
                _ = tick_interval.tick() => domain::Event::Tick,

                // on the frame interval add a render event to the que
                _ = frame_interval.tick() => domain::Event::Render,

                // Poll the crossterm backend terminal stream for the next crossterm terminal
                // event and send matched event to the que
                crossterm_event = crossterm_event_stream.next().fuse() => match crossterm_event {
                    Some(Ok(event)) => match event {
                        CrosstermEvent::FocusGained => domain::Event::FocusGained,
                        CrosstermEvent::FocusLost => domain::Event::FocusLost,
                        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => domain::Event::Key(key),
                        CrosstermEvent::Mouse(mouse) => domain::Event::Mouse(mouse),
                        CrosstermEvent::Paste(s) => domain::Event::Paste(s),
                        CrosstermEvent::Resize(x, y) => domain::Event::Resize(x, y),
                        _ => continue, // ignore other events
                    }
                    Some(Err(_)) => domain::Event::Error, // TODO: Pass error in
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

    /// Get the next event in the que
    pub async fn next(&mut self) -> Result<domain::Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Error::Generic("IO Crossterm Error".to_string()))
    }
}
