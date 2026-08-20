# XZXT RISC-V Bare-Metal Firmware (`xzxt-firmware`)

<div align="center">

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Architecture: RISC-V RV64](https://img.shields.io/badge/architecture-RISC--V%20RV64-blue.svg)](https://riscv.org/)
[![Nix Shell](https://img.shields.io/badge/nix-shell-blueviolet.svg)](https://nixos.org/)

*Developed by **Gooneymart™***

</div>

## Overview

`xzxt-firmware` is a modular bare-metal Rust firmware project designed to test and exercise the **XZXT custom RISC-V extension**.

The firmware implements **Type-R custom instructions** using the standard RISC-V `custom-0` opcode (`0x0B`). It targets a 64-bit **RV64** architecture using the `riscv64imac-unknown-none-elf` target and boots from a physical reset vector at `0x80000000`.

The firmware acts as the software test payload for **dual-target co-simulation**, allowing the same binary workload to be executed against:

1. **`gooney-emu`** — the software golden-model RISC-V emulator.
2. **Verilator** — the RTL/hardware simulation model.

This provides a common firmware workload for validating that the XZXT extension behaves consistently between the software and hardware implementations.

---

## Features

* Bare-metal Rust firmware with no operating system dependency.
* Targets **RV64IMAC**.
* Implements XZXT custom instructions through the `custom-0` opcode space.
* Uses inline RISC-V assembly for custom instruction wrappers.
* Boots from physical address `0x80000000`.
* Provides a dedicated linker script for the firmware memory layout.
* Produces both ELF and flat binary artifacts.
* Designed for software/hardware co-simulation.
* Reproducible development environment through **Nix**.

---

## Target Architecture

| Property             | Value                          |
| -------------------- | ------------------------------ |
| Architecture         | RISC-V 64-bit                  |
| ISA                  | RV64IMAC                       |
| Rust Target          | `riscv64imac-unknown-none-elf` |
| Custom Opcode        | `custom-0`                     |
| Opcode Value         | `0x0B`                         |
| Boot / Reset Address | `0x80000000`                   |
| ROM Base             | `0x80000000`                   |
| RAM Base             | `0x80010000                    |
| Binary Format        | Raw flat binary                |

---

## Project Structure

```text
xzxt-firmware/
├── .cargo/
│   └── config.toml          # Build target and linker configuration
├── link.ld                  # Bare-metal memory map
├── shell.nix                # Reproducible Nix development environment
├── Cargo.toml               # Package metadata and build profiles
└── src/
    ├── main.rs              # Firmware entry point and workload
    ├── custom.rs            # XZXT custom-0 instruction wrappers
    └── exit.rs              # Simulation exit traps and panic handler
```

### Source Files

#### `src/main.rs`

Contains the firmware entry point (`_start`) and the primary XZXT instruction test workload.

#### `src/custom.rs`

Provides Rust wrappers around the XZXT custom instructions using inline RISC-V assembly.

The implementation uses the RISC-V `custom-0` opcode space (`0x0B`) to encode XZXT Type-R instructions.

#### `src/exit.rs`

Provides simulation-specific exit handling, including memory-mapped exit traps and the bare-metal panic handler.

### Build Configuration

#### `.cargo/config.toml`

Defines the default Rust compilation target and linker configuration required to build the firmware for the bare-metal RISC-V environment.

#### `link.ld`

Defines the physical memory layout used by the firmware, including the ROM and RAM regions.

#### `shell.nix`

Provides the development environment and required RISC-V cross-compilation tools.

---

## Memory Layout

The firmware is linked using `link.ld` with the following physical memory layout:

```text
0x80000000  ┌─────────────────────────────┐
            │             ROM             │
            │       Firmware Image       │
            │                             │
0x80010000  ├─────────────────────────────┤
            │             RAM             │
            │      Runtime Data/Stack     │
            │                             │
            └─────────────────────────────┘
```

The generated firmware image is expected to be loaded at:

```text
0x80000000
```

Both simulation environments should therefore configure their firmware loader or memory model to place the flat binary at this address.

---

## Getting Started

### Prerequisites

The recommended development environment uses **Nix**.

Install:

* **Nix**
* Rust toolchain
* RISC-V cross-compilation tools provided by the Nix environment

The project is configured so that the required development dependencies can be initialized through `shell.nix`.

---

## 1. Enter the Development Environment

Start the reproducible Nix development shell:

```bash
nix-shell
```

The environment provides the Rust toolchain and RISC-V cross-binutils required to build and inspect the firmware.

---

## 2. Build the Firmware

Build the release firmware:

```bash
cargo build --release
```

The resulting ELF image will be generated at:

```text
target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

---

## 3. Generate the Flat Binary

The simulator and hardware model consume a raw firmware image rather than the ELF executable.

Create the test artifact directory:

```bash
mkdir -p tests
```

Then convert the ELF image into a flat binary:

```bash
riscv64-none-elf-objcopy \
    -O binary \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware \
    tests/firmware.bin
```

The resulting firmware image is:

```text
tests/firmware.bin
```

---

## Artifact Deliverables

After a successful build, the project produces two primary artifacts.

### ELF Image

```text
target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

The ELF image is useful for:

* Symbol-level debugging
* Disassembly
* Inspecting sections
* Examining generated machine code
* Debugging firmware execution

For example:

```bash
riscv64-none-elf-objdump \
    -d \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

### Flat Binary

```text
tests/firmware.bin
```

The flat binary is the image intended to be loaded directly into the simulated physical memory at:

```text
0x80000000
```

---

## Co-Simulation Workflow

The firmware is intended to provide the same workload to both the software emulator and the hardware model.

```text
                    ┌──────────────────────┐
                    │   xzxt-firmware      │
                    │                      │
                    │  XZXT Test Workload  │
                    └──────────┬───────────┘
                               │
                     tests/firmware.bin
                               │
              ┌────────────────┴────────────────┐
              │                                 │
              ▼                                 ▼
     ┌──────────────────┐              ┌──────────────────┐
     │    gooney-emu    │              │    Verilator     │
     │                  │              │                  │
     │ Software Golden  │              │ Hardware / RTL   │
     │     Model        │              │     Model        │
     └────────┬─────────┘              └────────┬─────────┘
              │                                 │
              └──────────────┬──────────────────┘
                             ▼
                    Compare XZXT Behavior
```

The objective is to verify that execution of the XZXT instructions produces consistent results across both implementations.

---

## XZXT Custom Instructions

XZXT instructions are encoded using the standard RISC-V `custom-0` opcode:

```text
Opcode: 0x0B
```

The firmware exposes these instructions through Rust wrappers in:

```text
src/custom.rs
```

This keeps the application-level workload in `main.rs` separate from the low-level instruction encoding and inline assembly implementation.

A typical execution flow is:

```text
Rust Workload
     │
     ▼
XZXT Instruction Wrapper
     │
     ▼
RISC-V Inline Assembly
     │
     ▼
custom-0 opcode (0x0B)
     │
     ├──────────────► gooney-emu
     │
     └──────────────► Verilator
```

---

## Build and Rebuild

To perform a clean release build:

```bash
cargo clean
cargo build --release
```

Then regenerate the flat binary:

```bash
mkdir -p tests

riscv64-none-elf-objcopy \
    -O binary \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware \
    tests/firmware.bin
```

---

## Inspecting the Firmware

### Inspect the ELF

```bash
riscv64-none-elf-readelf -h \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

### Inspect Sections

```bash
riscv64-none-elf-readelf -S \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

### Disassemble

```bash
riscv64-none-elf-objdump -d \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware
```

These tools are particularly useful when verifying that the compiler emitted the expected XZXT custom instructions.

---

## Validation

A successful firmware validation should confirm:

* The firmware builds for `riscv64imac-unknown-none-elf`.
* The ELF image is linked with the expected memory layout.
* The firmware starts execution at `0x80000000`.
* XZXT instructions are encoded using `custom-0` (`0x0B`).
* The flat binary can be loaded by `gooney-emu`.
* The same binary can be loaded by the Verilator model.
* XZXT instruction results match between the software and hardware implementations.
* The firmware reaches the expected simulation exit condition.

---

## Troubleshooting

### Missing RISC-V Toolchain

If `riscv64-none-elf-objcopy` cannot be found, make sure the Nix development environment is active:

```bash
nix-shell
```

Then verify:

```bash
which riscv64-none-elf-objcopy
```

### Missing Firmware Binary

If `tests/firmware.bin` does not exist, regenerate it from the release ELF:

```bash
mkdir -p tests

riscv64-none-elf-objcopy \
    -O binary \
    target/riscv64imac-unknown-none-elf/release/xzxt-firmware \
    tests/firmware.bin
```

### Incorrect Load Address

The firmware is linked for a physical boot address of:

```text
0x80000000
```

Ensure that the emulator or Verilator memory model loads the binary at this address.

Loading the image at a different address can cause incorrect startup behavior or invalid memory accesses.

---

## Development Notes

This project intentionally keeps the firmware small and deterministic so that it can be used as a repeatable validation payload for the XZXT extension.

When adding new XZXT instructions:

1. Add or update the instruction wrapper in `src/custom.rs`.
2. Add the corresponding workload or validation sequence in `src/main.rs`.
3. Rebuild the firmware.
4. Regenerate `tests/firmware.bin`.
5. Execute the firmware against `gooney-emu`.
6. Execute the same image against the Verilator model.
7. Compare the observed architectural results.

Keeping the same firmware image across both targets is important for meaningful co-simulation.

---

## License

This project is open-source software licensed under the **Mozilla Public License 2.0 (MPL 2.0)**.

See the project license file for the complete license terms.

---

## Corporate & Commercial Inquiries

For commercial utilization, proprietary embedding, or corporate licensing agreements concerning the XZXT extension and associated tooling, contact:

**Gooneymart®**

**Email:** `gooneymart@gmail.com`

---

## Project Summary

`xzxt-firmware` provides the bare-metal software workload used to validate the XZXT RISC-V extension across both software and hardware implementations.

```text
XZXT Extension
      │
      ▼
xzxt-firmware
      │
      ├──► gooney-emu
      │      Software Golden Model
      │
      └──► Verilator
             Hardware / RTL Model
```

The firmware therefore serves as the common execution payload for verifying **instruction encoding, execution semantics, memory behavior, and cross-model compatibility** of the XZXT extension.

