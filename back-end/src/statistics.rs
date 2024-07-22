use tracing::{event, Level};

pub(crate) struct Statistics {}

impl Default for Statistics {
    fn default() -> Self {
        Statistics::new()
    }
}

impl Statistics {
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn log_totals(&self) {
        event!(Level::INFO, "Statistics");
    }
}
