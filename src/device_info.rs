use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDeviceInfo {
    pub device_name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub serial_number: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub event_type: DeviceEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceEventType {
    Connected,
    Disconnected,
}

impl UsbDeviceInfo {
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

    pub fn format_plain(&self) -> String {
        let event_str = match self.event_type {
            DeviceEventType::Connected => "CONNECTED",
            DeviceEventType::Disconnected => "DISCONNECTED",
        };

        let serial_str = self
            .serial_number
            .as_ref()
            .map(|s| format!(" Serial: {}", s))
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
