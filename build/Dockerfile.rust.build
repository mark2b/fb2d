FROM rust-compiler

# configure cargo

ADD build/assets/cargo-config $HOME/.cargo/config

ENV PATH=$HOME/bin:$HOME/.cargo/bin:$PATH

USER root

RUN mkdir -p /source
RUN chown -R  build:build /source

USER build

VOLUME ["/source"]

RUN mkdir -p /source
RUN mkdir -p /source


VOLUME ["/build/.cargo/git", "/build/.cargo/registry"]

WORKDIR /source

ENTRYPOINT ["/bin/bash", "-c"]

