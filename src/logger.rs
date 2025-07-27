//! Event logging and output formatting for USB device monitoring.
//!
//! This module provides functionality for logging USB device events to console
//! and files in various formats (plain text and JSON).
//!
//! ## Notes
//!
//! - JSON output uses serde serialization; device handles are excluded from JSON.
//! - File logging respects system file permissions and will fail if permissions are insufficient.

use crate::device_info::UsbDeviceInfo;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::sync::mpsc;

/// Configuration and state for logging USB device events.
///
/// The logger handles output formatting and can write to both console
/// and log files simultaneously.
pub struct Logger {
    output_json: bool,
    log_file: Option<std::fs::File>,
    colorful: bool,
}

impl Logger {
    /// Creates a new logger instance.
    ///
    /// # Arguments
    ///
    /// * `output_json` - Whether to format output as JSON
    /// * `log_file_path` - Optional path to a log file
    ///
    /// # Errors
    ///
    /// Returns an error if the log file cannot be created or opened.
    ///
    /// # Examples
    ///
    /// ```
    /// use usbwatch_rs::logger::Logger;
    ///
    /// // Console-only logger with plain text
    /// let logger = Logger::new(false, None)?;
    ///
    /// // JSON logger with file output
    /// let logger = Logger::new(true, Some("usb-events.json"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        output_json: bool,
        log_file_path: Option<&str>,
        colorful: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let log_file = if let Some(path) = log_file_path {
            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .map_err(|e| format!("Failed to open log file '{path}': {e}"))?,
            )
        } else {
            None
        };

        Ok(Self {
            output_json,
            log_file,
            colorful,
        })
    }

    /// Logs a USB device event to console and file (if configured).
    ///
    /// The output format depends on the `output_json` setting configured
    /// during logger creation.
    ///
    /// # Arguments
    ///
    /// * `device_info` - Information about the USB device event
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialisation fails or file writing fails.
    pub fn log_device_event(
        &mut self,
        device_info: &UsbDeviceInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.output_json {
            let json = serde_json::to_string(device_info)?;
            println!("{}", json);
            if let Some(file) = &mut self.log_file {
                writeln!(file, "{}", json)?;
                file.flush()?;
            }
        } else {
            use anstyle::{AnsiColor, Style, WriteAnsi};
            let event_icon = match device_info.event_type {
                crate::device_info::DeviceEventType::Connected => "🔌",
                crate::device_info::DeviceEventType::Disconnected => "❌",
            };
            let style = if self.colorful {
                match device_info.event_type {
                    crate::device_info::DeviceEventType::Connected => {
                        Style::new().fg_color(Some(AnsiColor::Green))
                    }
                    crate::device_info::DeviceEventType::Disconnected => {
                        Style::new().fg_color(Some(AnsiColor::Red))
                    }
                }
            } else {
                Style::new()
            };
            let mut buf = Vec::new();
            write!(buf, "{} ", event_icon)?;
            style.write_ansi(&mut buf)?;
            write!(
                buf,
                "{} | VID: {} PID: {} | Serial: {} | Event: {:?} | {}",
                device_info.device_name,
                device_info.vendor_id,
                device_info.product_id,
                device_info.serial_number.as_deref().unwrap_or("-"),
                device_info.event_type,
                device_info.timestamp
            )?;
            let output = String::from_utf8_lossy(&buf);
            println!("{}", output);
            if let Some(file) = &mut self.log_file {
                writeln!(file, "{}", output)?;
                file.flush()?;
            }
        }
        Ok(())
    }
}

/// Async task that processes USB device events from a channel.
///
/// This function runs indefinitely, receiving device events and logging
/// them using the provided logger instance.
///
/// # Arguments
///
/// * `rx` - Receiver channel for USB device events
/// * `logger` - Logger instance for formatting and outputting events
pub async fn logger_task(mut rx: mpsc::Receiver<UsbDeviceInfo>, mut logger: Logger) {
    while let Some(device_info) = rx.recv().await {
        if let Err(e) = logger.log_device_event(&device_info) {
            eprintln!("Error logging device event: {e}");
        }
    }
}
