#!/bin/bash
cd /root/dev/rust/usbwatch-rs
echo "Building for Windows target..."
cargo build --target x86_64-pc-windows-gnu 2>&1 | tee windows_build.log
echo "Build completed. Check windows_build.log for details."
