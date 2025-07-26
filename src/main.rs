mod device_info;
mod logger;
mod watcher;

use clap::Parser;
use logger::{Logger, logger_task};
use tokio::sync::mpsc;
use watcher::UsbWatcher;

#[derive(Parser)]
#[command(name = "usbwatch-rs")]
#[command(about = "A cross-platform USB device monitor")]
#[command(version = "0.1.0")]
struct Cli {
    /// Output events in JSON format
    #[arg(long)]
    json: bool,

    /// Log events to file
    #[arg(long, value_name = "PATH")]
    logfile: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("🔌 USB Device Monitor - usbwatch-rs v0.1.0");
    println!("Press Ctrl+C to stop monitoring...");

    // Create channel for device events
    let (tx, rx) = mpsc::channel(100);

    // Initialise logger
    let logger = Logger::new(cli.json, cli.logfile.as_deref())?;

    // Start logger task
    let logger_handle = tokio::spawn(logger_task(rx, logger));

    // Create and start USB watcher
    let watcher = UsbWatcher::new(tx)?;

    // Handle Ctrl+C gracefully
    let watcher_handle = tokio::spawn(async move {
        if let Err(e) = watcher.start_monitoring().await {
            eprintln!("USB monitoring error: {}", e);
        }
    });

    // Wait for Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("\n📡 Shutting down USB monitor...");
        }
        _ = watcher_handle => {
            println!("📡 USB monitoring stopped");
        }
    }

    // Cleanup
    logger_handle.abort();

    Ok(())
}
