use std::collections::VecDeque;

use crate::domain;



#[derive(Debug, Clone, PartialEq)]
pub struct ToastState {
    /// Toast queue to store messages as they are triggered
    pub queue: VecDeque<domain::Toast>,

    /// The toast message being displayed. It can be none, so it is within an Option
    pub current: Option<domain::Toast>,
}

impl Default for ToastState {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        Self::new()
    }
}

impl ToastState {
    /// Construct a new application state instance
    pub fn new() -> Self {
        let queue = VecDeque::new();
        let current = None;

        Self {
            queue,
            current
        }
    }
}