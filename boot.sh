#!/bin/sh

if cargo bootimage; then
    echo "Completed image creation"
else
    echo "No bootimage installed, installing now"
    cargo install bootimage
    return
fi
qemu-system-x86_64 -drive format=raw,file=target/x86_64-hobby_os/debug/bootimage-hobby_os.bin
