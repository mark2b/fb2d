FROM debian:stretch

# update system
RUN apt-get update
RUN apt-get install -y curl git gcc g++ xz-utils wget make

ENV HOME=/build

RUN groupadd --system build && useradd --create-home --system --gid build --uid 1000 --home $HOME build;
