// Integration test for usbwatch-rs
// This test will only run on Linux for now

use tokio::sync::mpsc;
use usbwatch_rs::UsbWatcher;

#[tokio::test]
async fn test_usbwatcher_start_monitoring() {
    let (tx, mut rx) = mpsc::channel(10);
    let watcher = UsbWatcher::new(tx).expect("Failed to create watcher");
    tokio::spawn(async move {
        let _ = watcher.start_monitoring().await;
    });
    // Wait for a short time to see if any events are received
    for _ in 0..5 {
        if let Some(_event) = rx.recv().await {
            break;
        }
    }
    // We can't guarantee a device event, but the test should run without panicking
    // The test passes if it runs without panicking
    // The test passes if it runs without panicking
}
