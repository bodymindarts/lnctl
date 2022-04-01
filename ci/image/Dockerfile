FROM rust:latest
# see https://github.com/neomantra/docker-flatbuffers#usage-as-an-image-build-stage
COPY --from=neomantra/flatbuffers /usr/local/bin/flatc /usr/local/bin/flatc

RUN rustc --version

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN git clone https://github.com/bats-core/bats-core.git \
            && cd bats-core \
            && ./install.sh /usr/local

