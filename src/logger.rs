use crate::device_info::UsbDeviceInfo;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::sync::mpsc;

pub struct Logger {
    output_json: bool,
    log_file: Option<std::fs::File>,
}

impl Logger {
    pub fn new(
        output_json: bool,
        log_file_path: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let log_file = if let Some(path) = log_file_path {
            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .map_err(|e| format!("Failed to open log file '{}': {}", path, e))?,
            )
        } else {
            None
        };

        Ok(Self {
            output_json,
            log_file,
        })
    }

    pub fn log_device_event(
        &mut self,
        device_info: &UsbDeviceInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output = if self.output_json {
            serde_json::to_string(device_info)?
        } else {
            device_info.format_plain()
        };

        // Print to stdout
        println!("{}", output);

        // Write to log file if specified
        if let Some(ref mut file) = self.log_file {
            writeln!(file, "{}", output)?;
            file.flush()?;
        }

        Ok(())
    }
}

pub async fn logger_task(mut rx: mpsc::Receiver<UsbDeviceInfo>, mut logger: Logger) {
    while let Some(device_info) = rx.recv().await {
        if let Err(e) = logger.log_device_event(&device_info) {
            eprintln!("Error logging device event: {}", e);
        }
    }
}
