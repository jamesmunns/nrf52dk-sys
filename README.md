# nRF52dk-sys

This is a (work in progress) rust crate to support the [Nordic nRF52 Development Kit](https://www.nordicsemi.com/eng/Products/Bluetooth-low-energy/nRF52-DK) using Rust.

This crate uses the Nordic SoftDevice S132 (a binary Bluetooth stack), as well as the Nordic SDK (an open source hardware abstraction layer), which are written in C, and provides Rust bindings for those items.

This project aims to be a reference on how to combine C and Rust components, in order to create a Bluetooth peripheral which uses Rust for the main application software.

## Software Prerequisites

This project requires the following tools before building:

| Tool              | Recommended Version | Link/Install                                                                              |
| :---------------- | :------------------ | :---------------------------------------------------------------------------------------- |
| Clang             | 3.9                 | [debian/ubuntu](http://apt.llvm.org/) or [source](http://releases.llvm.org/download.html) |
| arm-none-eabi-gcc | 6.1                 | [Current Version](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)   |
| Rust (nightly)    | nightly-2017-11-15  | [rustup.rs](https://www.rustup.rs/)                                                       |
| Bindgen           | 0.31.3              | `cargo install bindgen --vers 0.31.3`                                                     |

If you would like more detailed installation instructions, please look at [The Detailed Setup Instructions](./SETUP.md).

If you use `docker`, please see the debian based [Dockerfile](./Dockerfile).

Additionally, the following tools are required to run or debug the firmware:

| Tool       | Recommended Version | Link/Install                                                                               |
| ---------- | ------------------- | ------------------------------------------------------------------------------------------ |
| SoftDevice | S132-v4.0.2         | [Nordic Download](http://www.nordicsemi.com/eng/nordic/Products/nRF52832/S132-SD-v4/58803) |
| JLink      | v6.16               | [JLink Download](https://www.segger.com/downloads/jlink)                                   |

## Building

```text
git clone --recursive https://github.com/jamesmunns/nrf52dk-sys
cd nrf52dk-sys
cargo build --example blinky
      Compiling core v0.0.0 (file:///root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore)
       Finished release [optimized] target(s) in 14.8 secs
       Updating registry `https://github.com/rust-lang/crates.io-index`
    Downloading r0 v0.2.1
    Downloading cortex-m v0.1.6
    Downloading volatile-register v0.1.2
      Compiling volatile-register v0.1.2
      Compiling gcc v0.3.50
      Compiling r0 v0.2.1
      Compiling cortex-m v0.1.6
      Compiling smooth_blue v0.1.0 (file:///nrf52dk-sys)
       Finished dev [unoptimized + debuginfo] target(s) in 15.66 secs
```

**NOTE:** This crate does not work with incremental compilation. In our
**.cargo/config**, we set `incremental = false`, which corresponds to setting
`CARGO_INCREMENTAL=0` in your environment.

## Flashing, Debugging, and Running

### Flashing the SoftDevice

First, you must flash the Nordic Soft Device. This should only be necessary to do once (the rest of the firmware will not overwrite this section).

```text
cd path/to/softdevice
JLinkExe -device NRF52832_XXAA -if SWD -speed 4000 -autoconnect 1
    Device "NRF52832_XXAA" selected.
    Found SWD-DP with ID 0x2BA01477
    Found SWD-DP with ID 0x2BA01477
    AP-IDR: 0x24770011, Type: AHB-AP
    AHB-AP ROM: 0xE00FF000 (Base addr. of first ROM table)
    Found Cortex-M4 r0p1, Little endian.
    FPUnit: 6 code (BP) slots and 2 literal slots
    CoreSight components:
    ROMTbl 0 @ E00FF000
    ROMTbl 0 [0]: FFF0F000, CID: B105E00D, PID: 000BB00C SCS
    ROMTbl 0 [1]: FFF02000, CID: B105E00D, PID: 003BB002 DWT
    ROMTbl 0 [2]: FFF03000, CID: B105E00D, PID: 002BB003 FPB
    ROMTbl 0 [3]: FFF01000, CID: B105E00D, PID: 003BB001 ITM
    ROMTbl 0 [4]: FFF41000, CID: B105900D, PID: 000BB9A1 TPIU
    ROMTbl 0 [5]: FFF42000, CID: B105900D, PID: 000BB925 ETM
    Cortex-M4 identified.
J-Link>loadfile s132_nrf52_4.0.2_softdevice.hex
    Downloading file [s132_nrf52_4.0.2_softdevice.hex]...
    Comparing flash   [100%] Done.
    Verifying flash   [100%] Done.
    O.K.
```

### Flashing to device

This only flashes the firmware built above. If you would like to flash and debug, skip forward.

```text
cd nrf52dk-sys
arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/blinky target.hex
JLinkExe -device NRF52832_XXAA -if SWD -speed 4000 -autoconnect 1
J-Link>loadfile target.hex
    Downloading file [target.hex]...
    Comparing flash   [100%] Done.
    Verifying flash   [100%] Done.
    O.K.
```

It may be necessary to reset the device after closing JLink with `CTRL-c`.

### Flash and Debug

First, create a GDB server on one terminal:

```text
JLinkGDBServer -device NRF52832_XXAA -if SWD -speed 4000
# ...
# Connecting to target...Connected to target
# Waiting for GDB connection...
```

Then, in another terminal:

```text
cd nrf52dk-sys
arm-none-eabi-gdb -tui target/thumbv7em-none-eabihf/debug/examples/blinky
(gdb) target remote :2331
# ...
(gdb) monitor reset
# ...
(gdb) load
# ...
(gdb) monitor reset
# ...
(gdb) continue
# ...
```

## Docker

I wrote the `Dockerfile` in order to support CI. This is not done yet. For now if you would like to build it to verify the master branch builds:

```text
cd nrf52dk-sys

# Build the image
docker build -t nrf52dk .

# Run the image
docker run -t nrf52dk
```

If the docker container ran successfully, you should see something like this at the end:

```text
 Compiling smooth_blue v0.1.0 (file:///nrf52dk-sys)
  Finished dev [unoptimized + debuginfo] target(s) in 15.66 secs
```

## License

All Rust components are provided under the [MIT License](./LICENSE). Additional components provided by the Nordic nRF5-sdk contain additional licenses.
