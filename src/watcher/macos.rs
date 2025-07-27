//! macOS-specific USB device watcher implementation.
//!
//! This is a stub implementation. Real support should use IOKit bindings or polling /dev for USB events.

#[cfg(target_os = "macos")]
use crate::device_info::{DeviceEventType, DeviceHandle, UsbDeviceInfo};
#[cfg(target_os = "macos")]
use tokio::sync::mpsc;

#[cfg(target_os = "macos")]
pub struct MacosUsbWatcher {
    tx: mpsc::Sender<UsbDeviceInfo>,
}

#[cfg(target_os = "macos")]
impl MacosUsbWatcher {
    pub fn new(tx: mpsc::Sender<UsbDeviceInfo>) -> Self {
        Self { tx }
    }

    pub async fn start_monitoring(&self) -> Result<(), String> {
        println!("Starting USB device monitoring on macOS (stub)...");
        // TODO: Implement real macOS USB monitoring using IOKit or polling
        Ok(())
    }
}
