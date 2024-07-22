#![allow(clippy::unused_async)]

use axum::http::HeaderMap;
mod repository;
use headers::GITHUB_EVENT_HEADER;
use payload::Payload;
use push::PushEvent;
use repository::RepositoryEvent;
use tracing::{event, Level};

use crate::settings;

mod headers;
mod payload;
mod push;

pub async fn handle_webhook(
    headers: HeaderMap,
    payload: serde_json::Value,
) -> Result<(), color_eyre::Report> {
    let github_event_header_value = headers.get(GITHUB_EVENT_HEADER);

    event!(Level::ERROR, ?headers);

    match github_event_header_value.map(|v| v.to_str()).transpose() {
        Ok(Some("push")) => handle_push(serde_json::from_value(payload)?).await?,
        Ok(Some("repository.edited")) => {
            handle_repository(serde_json::from_value(payload)?).await?;
        },
        Ok(Some(github_event)) => {
            event!(
                Level::INFO,
                ?github_event,
                "Received GitHub event that we don't seem to handle"
            );
        },
        Ok(None) => {
            event!(Level::INFO, "Missing GitHub Event header");
        },
        Err(error) => {
            event!(Level::ERROR, ?error, "Unparsable GitHub Event header value");
        },
    }
    Ok(())
}

async fn handle_push(payload: PushEvent) -> Result<(), color_eyre::Report> {
    let default_branch =
        payload.r#ref == format!("refs/heads/{}", payload.repository.default_branch);

    if !default_branch {
        event!(
            Level::DEBUG,
            "Not working on the default branch, returning..."
        );
        return Ok(());
    }

    let settings_modified = payload.commits.iter().any(|commit| {
        commit
            .added
            .iter()
            .any(|added| added == settings::FILE_NAME)
            || commit
                .modified
                .iter()
                .any(|added| added == settings::FILE_NAME)
    });

    if !settings_modified {
        event!(
            Level::DEBUG,
            "No changes in {} detected, returning",
            settings::FILE_NAME
        );
        return Ok(());
    }

    sync_settings(payload)
}

async fn handle_repository(payload: RepositoryEvent) -> Result<(), color_eyre::Report> {
    match payload.action.as_str() {
        "edited" => {
            handle_repository_edited(payload).await?;
        },
        "created" => {
            handle_repository_created(payload).await?;
        },
        _ => {
            unreachable!()
        },
    }
    todo!()
}

async fn handle_repository_edited(payload: RepositoryEvent) -> Result<(), color_eyre::Report> {
    if let Some(old_default_branch) = payload
        .changes
        .as_ref()
        .and_then(|c| c.get("default_branch"))
    {
        event!(
            Level::DEBUG,
            "Default branch changed from '{}' to '{}'",
            old_default_branch.from,
            payload.repository.default_branch
        );
    } else {
        event!(Level::DEBUG, "Repository configuration was edited but the default branch was not affected, returning...");
        return Ok(());
    }

    sync_settings(payload)
}

async fn handle_repository_created(payload: RepositoryEvent) -> Result<(), color_eyre::Report> {
    sync_settings(payload)
}

#[allow(clippy::needless_pass_by_value)]
fn sync_settings(_payload: impl Into<Payload>) -> Result<(), color_eyre::Report> {
    todo!()
}
