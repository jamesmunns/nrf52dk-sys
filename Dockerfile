FROM debian:jessie

# Install OS dependencies:
#   - curl/wget:        obtain other installers
#   - build-essential:  contains lots of useful tools
#   - git-core:         needed to pull project source
#   - software-prop...: needed to add clang repo
#   - libc6-dev-i386:   32 bit headers
RUN apt-get update && \
    apt-get install -y \
        wget \
        curl \
        build-essential \
        git-core \
        software-properties-common \
        libc6-dev-i386

# Install Clang v3.9 for bindgen
RUN add-apt-repository "deb http://apt.llvm.org/jessie/ llvm-toolchain-jessie-3.9 main" && \
    wget -O - http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    apt-get update && \
    apt-get install -y llvm-3.9-dev libclang-3.9-dev clang-3.9

# Install GCC6.1 arm-none-eabi
RUN wget https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-rm/6_1-2017q1/gcc-arm-none-eabi-6-2017-q1-update-linux.tar.bz2 \
  -O /gcc.tar.bz2

RUN tar xf /gcc.tar.bz2
ENV PATH="/gcc-arm-none-eabi-6-2017-q1-update/bin:${PATH}"

# Install rust
RUN curl https://sh.rustup.rs -sSf > install_rust.sh
RUN /bin/bash /install_rust.sh -y --default-toolchain nightly-2017-11-15
ENV PATH="/root/.cargo/bin:${PATH}"

# Use Xargo for cross platform building
RUN cargo install xargo --vers 0.3.8

# Use Bindgen as a binary to generate headers
RUN cargo install bindgen --vers 0.31.3

# Add the rust-src component so we can build `core`
RUN rustup component add rust-src

# Pull down the latest code/submodules
RUN git clone --recursive https://github.com/jamesmunns/nrf52dk-sys --branch wez_pr

# Move to the git repo
WORKDIR /nrf52dk-sys

CMD ["xargo", "build", "--example", "ble_app_template", "--quiet"]
