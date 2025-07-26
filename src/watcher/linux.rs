#[cfg(target_os = "linux")]
use crate::device_info::{DeviceEventType, UsbDeviceInfo};
#[cfg(target_os = "linux")]
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::Path;
#[cfg(target_os = "linux")]
use tokio::sync::mpsc;

#[cfg(target_os = "linux")]
pub struct LinuxUsbWatcher {
    tx: mpsc::Sender<UsbDeviceInfo>,
}

#[cfg(target_os = "linux")]
impl LinuxUsbWatcher {
    pub fn new(tx: mpsc::Sender<UsbDeviceInfo>) -> Self {
        Self { tx }
    }

    pub async fn start_monitoring(&self) -> Result<(), String> {
        println!("Starting USB device monitoring on Linux...");

        // Simple polling approach - check /sys/bus/usb/devices periodically
        let mut known_devices: HashMap<String, UsbDeviceInfo> = HashMap::new();

        loop {
            match self.scan_usb_devices().await {
                Ok(current_devices) => {
                    let current_map: HashMap<String, UsbDeviceInfo> = current_devices
                        .into_iter()
                        .map(|d| {
                            let key = format!(
                                "{}:{}:{}",
                                d.vendor_id,
                                d.product_id,
                                d.serial_number.as_deref().unwrap_or("unknown")
                            );
                            (key, d)
                        })
                        .collect();

                    // Check for new devices (connected)
                    for (key, device) in &current_map {
                        if !known_devices.contains_key(key) {
                            let mut device_clone = device.clone();
                            device_clone.event_type = DeviceEventType::Connected;
                            if let Err(e) = self.tx.send(device_clone).await {
                                eprintln!("Failed to send device event: {}", e);
                            }
                        }
                    }

                    // Check for removed devices (disconnected)
                    for (key, device) in &known_devices {
                        if !current_map.contains_key(key) {
                            let mut device_clone = device.clone();
                            device_clone.event_type = DeviceEventType::Disconnected;
                            if let Err(e) = self.tx.send(device_clone).await {
                                eprintln!("Failed to send device event: {}", e);
                            }
                        }
                    }

                    known_devices = current_map;
                }
                Err(e) => {
                    eprintln!("Error scanning USB devices: {}", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    async fn scan_usb_devices(&self) -> Result<Vec<UsbDeviceInfo>, String> {
        let mut devices = Vec::new();
        let usb_devices_path = Path::new("/sys/bus/usb/devices");

        if !usb_devices_path.exists() {
            return Err(
                "USB devices path not found. Make sure you're running on Linux with USB support."
                    .to_string(),
            );
        }

        let entries = fs::read_dir(usb_devices_path).map_err(|e| e.to_string())?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            // Skip entries that don't look like USB devices (e.g., usb1, usb2, etc.)
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Look for actual USB devices and root hubs, but skip interfaces
                // USB devices: patterns like "1-1", "1-2", "2-1", etc. (devices connected to ports)
                // USB root hubs: "usb1", "usb2", etc.
                // Skip interfaces: "1-0:1.0", "2-0:1.0", etc.
                let is_device = (name.matches('-').count() == 1 && !name.contains(':'))
                    || name.starts_with("usb");
                let is_interface = name.contains(':');

                if is_device && !is_interface {
                    if let Ok(device_info) = self.parse_usb_device(&path).await {
                        // Skip devices with all zero VID/PID (typically means no actual device info)
                        if device_info.vendor_id != "0000" || device_info.product_id != "0000" {
                            devices.push(device_info);
                        }
                    } else {
                        println!("Failed to parse device: {}", name);
                    }
                }
            }
        }

        Ok(devices)
    }

    async fn parse_usb_device(&self, device_path: &Path) -> Result<UsbDeviceInfo, String> {
        let vendor_id = self
            .read_sys_file(device_path, "idVendor")
            .unwrap_or_else(|| "0000".to_string());
        let product_id = self
            .read_sys_file(device_path, "idProduct")
            .unwrap_or_else(|| "0000".to_string());

        let product_name = self
            .read_sys_file(device_path, "product")
            .unwrap_or_else(|| "Unknown Device".to_string());
        let manufacturer = self
            .read_sys_file(device_path, "manufacturer")
            .unwrap_or_default();
        let serial_number = self.read_sys_file(device_path, "serial");

        let device_name = if !manufacturer.is_empty() && !product_name.is_empty() {
            format!("{} {}", manufacturer, product_name)
        } else if !product_name.is_empty() {
            product_name
        } else {
            "Unknown Device".to_string()
        };

        Ok(UsbDeviceInfo::new(
            device_name,
            vendor_id,
            product_id,
            serial_number,
            DeviceEventType::Connected, // Will be updated by caller
        ))
    }
    fn read_sys_file(&self, device_path: &Path, filename: &str) -> Option<String> {
        let file_path = device_path.join(filename);
        fs::read_to_string(file_path)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }
}

#[cfg(not(target_os = "linux"))]
pub struct LinuxUsbWatcher;

#[cfg(not(target_os = "linux"))]
impl LinuxUsbWatcher {
    pub fn new(_tx: tokio::sync::mpsc::Sender<crate::device_info::UsbDeviceInfo>) -> Self {
        Self
    }

    pub async fn start_monitoring(&self) -> Result<(), String> {
        Err("Linux USB monitoring not available on this platform".to_string())
    }
}
