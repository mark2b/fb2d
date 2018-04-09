FROM rust-toolchain

# install rustup with raspberry target
RUN curl https://sh.rustup.rs -sSf > $HOME/install_rustup.sh
RUN sh $HOME/install_rustup.sh -y --default-toolchain 1.25.0
RUN $HOME/.cargo/bin/rustup target add arm-unknown-linux-gnueabihf


