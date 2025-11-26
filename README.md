# Runtime-Patchable Rust Microkernel (x86_64)

This code is my final project for the CS392M2 course at Boston University. It is a x86_64 microkernel written in Rust, with basic features like Input and Output handling, a shell and basic memory management. The commands of the shell are written in a scripting language, and are modifiable during runtime.

---

## Features

- Microkernel architecture targeting **x86_64**
- Bootloader + startup sequence
- Memory paging + heap allocation
- Interrupt handling (keyboard input, VGA text mode output)
- Interactive shell with command parsing
- Runtime-modifiable kernel programs via custom bytecode
- Rust-first implementation with a focus on safety and extensibility

---

## Requirements

To build or run the OS image, you need:

- **Rust (stable toolchain)**
- **Cargo**
- **qemu-system-x86_64** installed on your system

On Debian/Ubuntu:

```
sudo apt install qemu-system-x86
```

On macOS (with Homebrew):
```
brew install qemu
```
Build & Run (Source)

You can build and launch the OS directly with Cargo:
```
cargo run
```
This will compile the kernel and automatically start QEMU with the resulting image.
Run the Prebuilt Image

The repository includes a precompiled raw disk image. You can run it manually with:
```
qemu-system-x86_64 -drive format=raw,file=zita_os.bin
```
Active development — features may change, especially around kernel bytecode semantics and runtime patching mechanisms.
