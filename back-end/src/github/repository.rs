use std::collections::HashMap;

use serde::Deserialize;

use super::push::Repository;

#[derive(Deserialize)]
pub(crate) struct RepositoryEvent {
    pub(crate) action: String,
    pub(crate) changes: Option<HashMap<String, ChangedFrom>>,
    pub(crate) repository: Repository,
}

#[derive(Deserialize)]
pub(crate) struct ChangedFrom {
    pub(crate) from: String,
}
