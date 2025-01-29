use anyhow::bail;
use axum::Router;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::info;

pub struct AppStarter {
    pub started_latch: Sender<SocketAddr>,
    pub shutdown_signal: Receiver<()>,
    pub terminated_latch: Sender<()>,
}

pub struct AppControl {
    pub started_latch: Receiver<SocketAddr>,
    pub shutdown_signal: Sender<()>,
    pub terminated_latch: Receiver<()>,
}

impl AppControl {
    pub async fn shutdown_and_await(
        shutdown_signal: Sender<()>,
        terminated_latch: Receiver<()>,
    ) -> anyhow::Result<()> {
        if let Err(_) = shutdown_signal.send(()) {
            bail!("Couldn't send shutdown signal");
        }
        terminated_latch.await?;
        Ok(())
    }
}

impl AppStarter {
    pub fn new_with_latches() -> (AppStarter, AppControl) {
        let (started_tx, started_rx) = oneshot::channel::<_>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let (terminated_tx, terminated_rx) = oneshot::channel::<()>();

        (
            AppStarter {
                started_latch: started_tx,
                shutdown_signal: shutdown_rx,
                terminated_latch: terminated_tx,
            },
            AppControl {
                started_latch: started_rx,
                shutdown_signal: shutdown_tx,
                terminated_latch: terminated_rx,
            },
        )
    }

    pub async fn start(self, addr: String, app: Router) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let bound_addr: SocketAddr = listener.local_addr()?;
        info!("Listening on {}", bound_addr);

        self.started_latch.send(bound_addr).ok();

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(self.shutdown_signal))
            .await?;

        info!("Terminated application");
        self.terminated_latch.send(()).ok();

        Ok(())
    }
}

async fn shutdown_signal(receiver: Receiver<()>) {
    receiver.await.ok();
    info!("Shutdown signal received");
}
