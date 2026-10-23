use anyhow::Result;
use tokio::time::{Duration, sleep};

mod cla;
mod notifier;
mod provider;
mod signals;
mod tracker;

#[tokio::main]
async fn main() -> Result<()> {
    println!("= sunshine-couch-notifier =");
    println!("service starting ...");

    let provider_mode = cla::provider_mode_from_args()?;
    let mut log_provider = provider::new_log(provider_mode)?;
    let mut info_tracker = tracker::Info::new();
    println!("service started");

    loop {
        tokio::select! {
            // execute the shutdown signals
            () = signals::shutdown_signal() => {
                println!("service stopping ...");
                log_provider.terminate().await;
                break;
            }

            // otherwise, process the log
            next_line = log_provider.next_line() => {
                let ok_line = next_line?;

                if let Some(line) = ok_line {
                    info_tracker.process_line(&line);
                } else {
                    // slightly less busy loop, if input is idle
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    println!("service stopped");
    Ok(())
}
