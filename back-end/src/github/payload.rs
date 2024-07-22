use crate::github::{push::PushEvent, repository::RepositoryEvent};

#[allow(dead_code)]
pub(crate) enum Payload {
    PushEvent(PushEvent),
    RepositoryEvent(RepositoryEvent),
}

impl From<RepositoryEvent> for Payload {
    fn from(value: RepositoryEvent) -> Self {
        Payload::RepositoryEvent(value)
    }
}

impl From<PushEvent> for Payload {
    fn from(value: PushEvent) -> Self {
        Payload::PushEvent(value)
    }
}
