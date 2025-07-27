//! # USBWatch
//!
//! A cross-platform USB device monitoring library and command-line tool.
//!
//! USBWatch provides real-time monitoring of USB device connection and
//! disconnection events on Linux and Windows systems. It offers both
//! a library API for integration into other applications and a standalone
//! command-line tool.
//!
//! ## Features
//!
//! - **Cross-platform**: Works on Linux (sysfs) and Windows (Win32 APIs)
//! - **Real-time monitoring**: Detect USB events as they happen
//! - **Multiple output formats**: Plain text and JSON
//! - **File logging**: Save events to log files
//! - **Async/await support**: Built with Tokio for efficient I/O
//!
//! ## Quick Start
//!
//! ### Command Line Usage
//!
//! ```bash
//! # Monitor USB devices with default output
//! usbwatch
//!
//! # Monitor with JSON output
//! usbwatch --json
//!
//! # Monitor and log to file
//! usbwatch --logfile usb-events.log
//! ```
//!
//! ### Library Usage
//!
//! ```rust,no_run
//! use usbwatch_rs::{UsbWatcher, UsbDeviceInfo};
//! use tokio::sync::mpsc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (tx, mut rx) = mpsc::channel(100);
//!     let watcher = UsbWatcher::new(tx)?;
//!
//!     // Start monitoring in a background task
//!     tokio::spawn(async move {
//!         if let Err(e) = watcher.start_monitoring().await {
//!             eprintln!("Monitoring error: {}", e);
//!         }
//!     });
//!
//!     // Process device events
//!     while let Some(device_info) = rx.recv().await {
//!         println!("Device event: {}", device_info);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Platform Support
//!
//! - **Linux**: Uses sysfs filesystem (`/sys/bus/usb/devices`)
//! - **Windows**: Uses Win32 Device Installation APIs
//!
//! ## Error Handling
//!
//! All public APIs use `Result` types for proper error handling. Platform-specific
//! errors are wrapped in boxed `std::error::Error` for consistency.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod device_info;
pub mod logger;
pub mod watcher;

// Re-export commonly used types
pub use device_info::{DeviceEventType, UsbDeviceInfo};
pub use logger::{logger_task, Logger};
pub use watcher::UsbWatcher;

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
