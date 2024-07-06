#![allow(clippy::module_name_repetitions)]

use std::num::NonZeroU16;

use tracing::{event, Level};

pub(crate) const DEFAULT_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(2223) };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub(crate) port: NonZeroU16,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub(crate) fn new() -> Self {
        Self { port: DEFAULT_PORT }
    }

    pub(crate) fn set_port(&mut self, port: NonZeroU16) {
        self.port = port;
    }

    pub(crate) fn log(&self) {
        event!(Level::INFO, "Port: {}", self.port);
    }
}
