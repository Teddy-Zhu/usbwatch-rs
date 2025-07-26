mod device_info;
mod logger;
mod watcher;

use clap::{Parser, Subcommand};
use logger::{Logger, logger_task};
use std::env;
use std::fs;
use std::path::Path;
use tokio::sync::mpsc;
use watcher::UsbWatcher;

#[derive(Parser)]
#[command(name = "usbwatch")]
#[command(about = "A cross-platform USB device monitor")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Output events in JSON format (monitor mode only)
    #[arg(long, global = true)]
    json: bool,

    /// Log events to file (monitor mode only)
    #[arg(long, value_name = "PATH", global = true)]
    logfile: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor USB device events (default)
    Monitor,
    /// Install usbwatch to system PATH
    Install,
    /// Uninstall usbwatch from system PATH
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Monitor) {
        Commands::Monitor => run_monitor(cli.json, cli.logfile).await,
        Commands::Install => install_binary(),
        Commands::Uninstall => uninstall_binary(),
    }
}

async fn run_monitor(
    json: bool,
    logfile: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 USB Device Monitor - usbwatch v0.1.0");
    println!("Press Ctrl+C to stop monitoring...");

    // Create channel for device events
    let (tx, rx) = mpsc::channel(100);

    // Initialise logger
    let logger = Logger::new(json, logfile.as_deref())?;

    // Start logger task
    let logger_handle = tokio::spawn(logger_task(rx, logger));

    // Create and start USB watcher
    let watcher = UsbWatcher::new(tx)?;

    // Handle Ctrl+C gracefully
    let watcher_handle = tokio::spawn(async move {
        if let Err(e) = watcher.start_monitoring().await {
            eprintln!("USB monitoring error: {e}");
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

fn install_binary() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = env::current_exe()?;
    let exe_name = if cfg!(windows) {
        "usbwatch.exe"
    } else {
        "usbwatch"
    };

    // Determine target directory
    let target_dir = if cfg!(windows) {
        // On Windows, try to install to a directory in PATH
        let program_files =
            env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
        Path::new(&program_files).join("usbwatch")
    } else {
        // On Unix-like systems, install to /usr/local/bin
        Path::new("/usr/local/bin").to_path_buf()
    };

    let target_path = target_dir.join(exe_name);

    // Create target directory if it doesn't exist (Windows only)
    if cfg!(windows) {
        fs::create_dir_all(&target_dir)?;
    }

    // Copy the binary
    fs::copy(&current_exe, &target_path)?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_path, perms)?;
    }

    println!(
        "✅ Successfully installed usbwatch to {}",
        target_path.display()
    );

    if cfg!(windows) {
        println!(
            "📝 Note: You may need to add {} to your PATH environment variable",
            target_dir.display()
        );
    }

    println!("🚀 You can now run 'usbwatch' from anywhere!");

    Ok(())
}

fn uninstall_binary() -> Result<(), Box<dyn std::error::Error>> {
    let exe_name = if cfg!(windows) {
        "usbwatch.exe"
    } else {
        "usbwatch"
    };

    // Determine target directory
    let target_dir = if cfg!(windows) {
        let program_files =
            env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
        Path::new(&program_files).join("usbwatch")
    } else {
        Path::new("/usr/local/bin").to_path_buf()
    };

    let target_path = target_dir.join(exe_name);

    if target_path.exists() {
        fs::remove_file(&target_path)?;
        println!(
            "✅ Successfully uninstalled usbwatch from {}",
            target_path.display()
        );

        // Remove directory on Windows if empty
        if cfg!(windows) && target_dir.read_dir()?.next().is_none() {
            fs::remove_dir(&target_dir)?;
            println!("🗑️  Removed empty directory {}", target_dir.display());
        }
    } else {
        println!("❌ usbwatch is not installed at {}", target_path.display());
        return Ok(());
    }

    println!("🧹 usbwatch has been uninstalled");

    Ok(())
}
