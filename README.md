# Hobby-OS
This is an x86-64 OS written in Rust based around the info on Philipp Oppermann's blog. This is a work in progress. 

# How to boot
With Qemu
```
./boot.sh
```
To boot on hardware use the following command and replace sdX with the appropriate device. Be careful to use the right device.
```
dd if=target/x86_64-hobby_os/debug/bootimage-hobby_os.bin of=/dev/sdX && sync
```