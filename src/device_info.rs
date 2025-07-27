//! USB device information structures and event types.
//!
//! This module provides the core data structures for representing USB device
//! information and events in the usbwatch monitoring system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Information about a USB device and its connection event.
///
/// This structure contains all relevant metadata about a USB device,
/// including identification information and the timestamp when the
/// event occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDeviceInfo {
    /// Human-readable name of the device
    pub device_name: String,
    /// USB Vendor ID in hexadecimal format (e.g., "1d6b")
    pub vendor_id: String,
    /// USB Product ID in hexadecimal format (e.g., "0002")
    pub product_id: String,
    /// Optional serial number of the device
    pub serial_number: Option<String>,
    /// UTC timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Type of device event (connected or disconnected)
    pub event_type: DeviceEventType,
}

/// Types of USB device events that can be monitored.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceEventType {
    /// Device was connected to the system
    Connected,
    /// Device was disconnected from the system
    Disconnected,
}

impl UsbDeviceInfo {
    /// Creates a new USB device information record.
    ///
    /// The timestamp is automatically set to the current UTC time.
    ///
    /// # Arguments
    ///
    /// * `device_name` - Human-readable name of the device
    /// * `vendor_id` - USB Vendor ID in hexadecimal format
    /// * `product_id` - USB Product ID in hexadecimal format
    /// * `serial_number` - Optional serial number
    /// * `event_type` - Type of device event
    ///
    /// # Examples
    ///
    /// ```
    /// use usbwatch_rs::device_info::{UsbDeviceInfo, DeviceEventType};
    ///
    /// let device = UsbDeviceInfo::new(
    ///     "USB Storage Device".to_string(),
    ///     "0781".to_string(),
    ///     "5583".to_string(),
    ///     Some("1234567890".to_string()),
    ///     DeviceEventType::Connected,
    /// );
    /// ```
    pub fn new(
        device_name: String,
        vendor_id: String,
        product_id: String,
        serial_number: Option<String>,
        event_type: DeviceEventType,
    ) -> Self {
        Self {
            device_name,
            vendor_id,
            product_id,
            serial_number,
            timestamp: Utc::now(),
            event_type,
        }
    }

    /// Formats the device information as a human-readable string.
    ///
    /// Returns a formatted string suitable for console output or log files.
    /// The format includes timestamp, event type, device name, VID/PID, and
    /// optional serial number.
    ///
    /// # Examples
    ///
    /// ```
    /// use usbwatch_rs::device_info::{UsbDeviceInfo, DeviceEventType};
    ///
    /// let device = UsbDeviceInfo::new(
    ///     "USB Storage".to_string(),
    ///     "0781".to_string(),
    ///     "5583".to_string(),
    ///     None,
    ///     DeviceEventType::Connected,
    /// );
    ///
    /// let formatted = device.format_plain();
    /// // Output: "[2025-07-27 10:30:15 UTC] CONNECTED - USB Storage (VID: 0781, PID: 5583)"
    /// ```
    pub fn format_plain(&self) -> String {
        let event_str = match self.event_type {
            DeviceEventType::Connected => "CONNECTED",
            DeviceEventType::Disconnected => "DISCONNECTED",
        };

        let serial_str = self
            .serial_number
            .as_ref()
            .map(|s| format!(" Serial: {s}"))
            .unwrap_or_default();

        format!(
            "[{}] {} - {} (VID: {}, PID: {}){}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            event_str,
            self.device_name,
            self.vendor_id,
            self.product_id,
            serial_str
        )
    }
}

impl std::fmt::Display for UsbDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_plain())
    }
}

impl std::fmt::Display for DeviceEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceEventType::Connected => write!(f, "Connected"),
            DeviceEventType::Disconnected => write!(f, "Disconnected"),
        }
    }
}
