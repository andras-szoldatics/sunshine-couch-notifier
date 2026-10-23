async fn ctrl_c_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[cfg(unix)]
async fn sigterm_signal() {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM signal handler")
        .recv()
        .await;
}

#[cfg(unix)]
pub async fn shutdown_signal() {
    tokio::select! {
        () = ctrl_c_signal() => {
            println!("CTRL+C signal received");
        },
        () = sigterm_signal() => {
            println!("SIGTERM signal received");
        },
    }
}

#[cfg(not(unix))]
pub async fn shutdown_signal() {
    tokio::select! {
        () = ctrl_c_signal() => {
            println!("CTRL+C signal received");
        },
    }
}
