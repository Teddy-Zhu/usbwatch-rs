//! Cross-platform USB device monitoring implementations.
//!
//! Platform-specific USB monitoring implementations for Linux, Windows, and macOS, abstracted behind a common `UsbWatcher` interface.
//! Uses sysfs (Linux), Win32 APIs (Windows), and IOKit (macOS).

/// Linux-specific USB monitoring implementation using sysfs.
#[cfg(target_os = "linux")]
pub mod linux;

/// Windows-specific USB monitoring implementation using Win32 APIs.
#[cfg(target_os = "windows")]
pub mod windows;

/// macOS-specific USB monitoring implementation using IOKit or polling.
#[cfg(target_os = "macos")]
pub mod macos;

use crate::device_info::UsbDeviceInfo;
use tokio::sync::mpsc;

/// Cross-platform USB device watcher.
///
/// This enum provides a unified interface for USB monitoring across
/// different operating systems. The appropriate implementation is
/// selected at compile time based on the target platform.
pub enum UsbWatcher {
    /// Windows implementation using Win32 APIs
    #[cfg(target_os = "windows")]
    Windows(windows::WindowsUsbWatcher),
    /// Linux implementation using sysfs
    #[cfg(target_os = "linux")]
    Linux(linux::LinuxUsbWatcher),
    /// macOS implementation using IOKit or polling
    #[cfg(target_os = "macos")]
    Macos(macos::MacosUsbWatcher),
    /// Placeholder for unsupported platforms
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    Unsupported,
}

impl UsbWatcher {
    /// Creates a new USB watcher for the current platform.
    ///
    /// # Arguments
    ///
    /// * `sender` - Channel sender for publishing device events
    ///
    /// # Errors
    ///
    /// Returns an error if the platform-specific watcher cannot be initialised.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use usbwatch_rs::UsbWatcher;
    /// use tokio::sync::mpsc;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, rx) = mpsc::channel(100);
    /// let watcher = UsbWatcher::new(tx)?;
    /// # Ok(())
    /// # }
    /// ```
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

        #[cfg(target_os = "macos")]
        {
            let watcher = macos::MacosUsbWatcher::new(sender);
            Ok(UsbWatcher::Macos(watcher))
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(UsbWatcher::Unsupported)
        }
    }

    /// Starts monitoring USB devices.
    ///
    /// This method runs indefinitely, monitoring for USB device connection
    /// and disconnection events. Events are sent through the channel provided
    /// during construction.
    ///
    /// # Errors
    ///
    /// Returns an error if monitoring cannot be started or if a critical
    /// error occurs during monitoring.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use usbwatch_rs::UsbWatcher;
    /// use tokio::sync::mpsc;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, mut rx) = mpsc::channel(100);
    /// let watcher = UsbWatcher::new(tx)?;
    ///
    /// // Start monitoring in background
    /// tokio::spawn(async move {
    ///     if let Err(e) = watcher.start_monitoring().await {
    ///         eprintln!("Monitoring error: {}", e);
    ///     }
    /// });
    ///
    /// // Process events
    /// while let Some(device_info) = rx.recv().await {
    ///     println!("USB event: {}", device_info);
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
                .map_err(|e| Box::new(std::io::Error::other(e)))?),
            #[cfg(target_os = "macos")]
            UsbWatcher::Macos(watcher) => Ok(watcher
                .start_monitoring()
                .await
                .map_err(|e| Box::new(std::io::Error::other(e)))?),
            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            UsbWatcher::Unsupported => Err("USB monitoring not supported on this platform".into()),
        }
    }
}
