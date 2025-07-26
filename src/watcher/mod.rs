#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

use crate::device_info::UsbDeviceInfo;
use tokio::sync::mpsc;

pub enum UsbWatcher {
    #[cfg(target_os = "windows")]
    Windows(windows::WindowsUsbWatcher),
    #[cfg(target_os = "linux")]
    Linux(linux::LinuxUsbWatcher),
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    Unsupported,
}

impl UsbWatcher {
    pub fn new(sender: mpsc::Sender<UsbDeviceInfo>) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            let watcher = windows::WindowsUsbWatcher::new(sender);
            Ok(UsbWatcher::Windows(watcher))
        }

        #[cfg(target_os = "linux")]
        {
            let watcher = linux::LinuxUsbWatcher::new(sender);
            Ok(UsbWatcher::Linux(watcher))
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Ok(UsbWatcher::Unsupported)
        }
    }

    pub async fn start_monitoring(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            #[cfg(target_os = "windows")]
            UsbWatcher::Windows(watcher) => Ok(watcher
                .start_monitoring()
                .await
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?),
            #[cfg(target_os = "linux")]
            UsbWatcher::Linux(watcher) => Ok(watcher
                .start_monitoring()
                .await
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?),
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            UsbWatcher::Unsupported => {
                eprintln!("USB monitoring not supported on this platform");
                Ok(())
            }
        }
    }
}
