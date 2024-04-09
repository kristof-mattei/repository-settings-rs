use tokio::io::{stdin, AsyncBufReadExt};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{event, Level};

pub(crate) fn monitor_stdin(sender: Sender<String>, token: CancellationToken) -> JoinHandle<()> {
    tokio::spawn(async move {
        let _guard = token.clone().drop_guard();

        let server = tokio::io::BufReader::new(stdin());

        let mut lines = server.lines();

        loop {
            let message = tokio::select! {
                () = token.cancelled() => {
                    // The token was cancelled
                    break;
                },
                result = lines.next_line() => match result {
                    Ok(Some(message)) => message,
                    Ok(None) => {
                        event!(Level::INFO, "Stdin is closed");
                        break;
                    },
                    Err(err) => {
                        event!(Level::ERROR, ?err, "Failure to read from stdin");
                        break;
                    }
                }
            };

            if let Err(err) = sender.send(message).await {
                event!(
                    Level::ERROR,
                    ?err,
                    "Failed to send message to mpsc, stopping..."
                );
                break;
            }
        }
    })
}
