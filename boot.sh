#!/bin/sh
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=/home/colton/Projects/hobby_os/target/x86_64-hobby_os/debug/bootimage-hobby_os.bin
