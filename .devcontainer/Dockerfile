FROM mcr.microsoft.com/vscode/devcontainers/cpp:0-ubuntu-22.04

RUN apt-get update --yes && apt-get --yes install pkg-config && \
    source="https://www.segger.com/downloads/jlink/JLink_Linux_V756_x86_64.deb" && \
    curl -o /root/jlink.deb -d accept_license_agreement=accepted -d non_emb_ctr=confirmed $source && \
    dpkg -i /root/jlink.deb && \
    rm /root/jlink.deb

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    rustup install 1.61.0 && \
    rustup component add llvm-tools-preview && \
    rustup component add clippy && \
    rustup target add thumbv7em-none-eabihf && \
    cargo install cargo-binutils && \
    cargo install cargo-watch 

RUN apt-get update --yes && apt-get --yes install gcc-arm-none-eabi gdb-multiarch gcc-multilib g++-multilib lldb
