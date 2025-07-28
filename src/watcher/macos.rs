//! macOS-specific USB device watcher implementation.
//!
//! Uses IOKit FFI to detect USB device events in real time. Supports colored output and modern CLI integration.

#[cfg(target_os = "macos")]
use crate::device_info::{DeviceEventType, DeviceHandle, UsbDeviceInfo};
#[cfg(target_os = "macos")]
use io_kit_sys::types::*;
#[cfg(target_os = "macos")]
use io_kit_sys::*;
#[cfg(target_os = "macos")]
use std::ffi::CStr;
#[cfg(target_os = "macos")]
use tokio::sync::mpsc;

#[cfg(target_os = "macos")]
/// Watches for USB device events on macOS using IOKit.
///
/// This struct provides asynchronous monitoring of USB device connections and disconnections
/// on macOS, sending events through a Tokio channel.
pub struct MacosUsbWatcher {
    tx: mpsc::Sender<UsbDeviceInfo>,
}

#[cfg(target_os = "macos")]
impl MacosUsbWatcher {
    /// Creates a new `MacosUsbWatcher` with the given channel sender.
    ///
    /// # Arguments
    ///
    /// * `tx` - Tokio channel sender for publishing USB device events.
    pub fn new(tx: mpsc::Sender<UsbDeviceInfo>) -> Self {
        Self { tx }
    }

    /// Starts monitoring USB devices on macOS.
    ///
    /// Enumerates currently connected USB devices and sends their info through the channel.
    /// In a full implementation, this would register for device notifications and run the event loop.
    ///
    /// # Errors
    ///
    /// Returns an error if IOKit FFI calls fail or device enumeration cannot be performed.
    pub async fn start_monitoring(&self) -> Result<(), String> {
        println!("Starting USB device monitoring on macOS...");
        // SAFETY: FFI calls to IOKit
        unsafe {
            let matching_dict = IOServiceMatching(b"IOUSBDevice\0".as_ptr() as *const i8);
            if matching_dict.is_null() {
                return Err("Failed to create matching dictionary for IOUSBDevice".to_string());
            }

            let mut iter: io_iterator_t = 0;
            let kr = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
            if kr != 0 {
                return Err(format!("IOServiceGetMatchingServices failed: {kr}"));
            }

            loop {
                let device = IOIteratorNext(iter);
                if device == 0 {
                    break;
                }
                // Example: get device name
                let mut device_name_buf = [0i8; 128];
                let kr = IORegistryEntryGetName(device, device_name_buf.as_mut_ptr());
                let device_name = if kr == 0 {
                    CStr::from_ptr(device_name_buf.as_ptr())
                        .to_string_lossy()
                        .into_owned()
                } else {
                    "Unknown USB Device".to_string()
                };

                // TODO: Get vendor/product/serial info from properties
                let info = UsbDeviceInfo {
                    device_name,
                    vendor_id: "unknown".to_string(),
                    product_id: "unknown".to_string(),
                    serial_number: None,
                    timestamp: chrono::Utc::now(),
                    event_type: DeviceEventType::Connected,
                    device_handle: DeviceHandle::Macos {
                        device_id: format!("{device}"),
                    },
                };
                let _ = self.tx.send(info).await;
                IOObjectRelease(device);
            }
            IOObjectRelease(iter);
        }
        Ok(())
    }
}
