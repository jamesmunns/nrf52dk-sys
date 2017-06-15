# Installation Instructions

The following will discuss in more detail what needs to be installed, and why. I suggest following these steps in this order.

This guide will be written against a clean debian jessie installation. Steps should be similar for other linux distributions, as well as OSX. If these steps do not work for you, please let me know! If you found a fix or would like to add more specific steps for other operating systems (especially OSX or Windows), please open a pull request!

## 1. General Prerequisites

These items are required for subsequent steps.

```bash
apt-get update
apt-get install -y wget curl build-essential git-core software-properties-common libc6-dev-i386
```

## 2. Clang/LLVM

Clang/LLVM is used by `bindgen` to generate the Rust bindings to C code. If your system already has it installed, or the package manager offers it, please make sure you have version 3.9. Other versions are not tested, and are unlikely to work.

```bash
# Add the repository and the necessary keys
add-apt-repository "deb http://apt.llvm.org/jessie/ llvm-toolchain-jessie-3.9 main"
wget -O - http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

# Update repositories and install Clang/LLVM
apt-get update
apt-get install -y llvm-3.9-dev libclang-3.9-dev clang-3.9
```

## 3. arm-none-eabi-gcc (and friends)

The embedded flavor of GCC is necessary to compile the existing C code, link together the C and Rust code, as well as flash and debug our software.

We will manually download and install GCC, as the the debian apt repos contain an older version which will not work for our purposes.

```bash
# obtain installer
wget https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-rm/6_1-2017q1/gcc-arm-none-eabi-6-2017-q1-update-linux.tar.bz2

# unzip
tar xf gcc-arm-none-eabi-6-2017-q1-update-linux.tar.bz2

# Move to a sane location (you may pick a different location)
sudo mv ./gcc-arm-none-eabi-6-2017-q1-update /opt

# Add this GCC to your path (this path must match the above path)
#   NOTE: if you want to make this permanent, add this line to the
#   end of your rc file (example: ~/.zshrc or ~/.bashrc)
export PATH=/opt/gcc-arm-none-eabi-6-2017-q1-update/bin
```

## 4a. Clean install of Rust

If you already have Rust installed, please skip ahead to step 4b.

This step will install Rust and set the default toolchain to the nightly.

Note: After performing this step, you may need to open a new terminal window for Rust to be added to your path.

```bash
curl https://sh.rustup.rs -sSf > install_rust.sh
/bin/bash /install_rust.sh -y --default-toolchain nightly-2017-06-12
```

## 4b. Rustup already installed

If you already have Rust installed via Rustup, you only need to install the correct toolchain version.

```bash
rustup install nightly-2017-06-12
rustup default nightly-2017-06-12
```

## 5. Install Xargo and Bindgen

Xargo makes compiling embedded crates easier. Bindgen automatically generates Rust bindings from C/C++ code.

```bash
cargo install xargo --vers 0.3.8
cargo install bindgen --vers 0.25.3
```

## 6. Install Rust Core Source

It is necessary to rebuild the `core` component of Rust for our target. Don't worry, `xargo` takes care of this, we just need to provide it Rust's source code.

```bash
rustup component add rust-src
```

## 7. Download the `nrf52dk-sys` repo, and build an example

You made it! Now to verify the install went well, lets checkout the `nrf52dk-sys` crate, and build one of the examples.

```bash
git clone --recursive https://github.com/jamesmunns/nrf52dk-sys
cd nrf52dk-sys
xargo build --example blinky
```

If everything went well, the last lines on your terminal should look like this:

```text
Compiling nrf52dk-sys v0.1.1 (file:///nrf52dk-sys)
 Finished dev [unoptimized + debuginfo] target(s) in 25.48 secs
```

## 8. Install nRF52 specific components

The above steps verify you can compile your firmware. Before interacting with actual hardware, you will need to download the following components. Instructions for what to do with these are on the [Main README](./README.md).

* [Nordic Download](http://www.nordicsemi.com/eng/nordic/Products/nRF52832/S132-SD-v4/58803)
* [JLink Download](https://www.segger.com/downloads/jlink)
