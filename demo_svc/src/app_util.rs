use axum::Router;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::info;

pub struct AppControl {
    pub started_latch: Sender<SocketAddr>,
    pub terminated_latch: Sender<()>,
}

pub struct AppLatches {
    pub started_latch: Receiver<SocketAddr>,
    pub terminated_latch: Receiver<()>,
}

impl AppControl {
    pub fn new_with_latches() -> (AppControl, AppLatches) {
        let (started_tx, started_rx) = oneshot::channel::<_>();
        let (terminated_tx, terminated_rx) = oneshot::channel::<()>();

        (
            AppControl {
                started_latch: started_tx,
                terminated_latch: terminated_tx,
            },
            AppLatches {
                started_latch: started_rx,
                terminated_latch: terminated_rx,
            },
        )
    }

    pub async fn start(self, port: u32, app: Router) -> anyhow::Result<()> {
        let addr = format!("0.0.0.0:{port}");
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let bound_addr: SocketAddr = listener.local_addr()?;
        info!("Listening on {}", bound_addr);

        // Test the latch
        #[cfg(test)]
        tokio::time::sleep(Duration::from_secs(1)).await;

        self.started_latch.send(bound_addr).ok();

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        self.terminated_latch.send(()).ok();

        Ok(())
    }
}

async fn shutdown_signal() {
    // thread::sleep(Duration::from_secs(5));
    tokio::time::sleep(Duration::from_secs(50)).await;
}
