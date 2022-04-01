FROM rust:latest AS build
  # see https://github.com/neomantra/docker-flatbuffers#usage-as-an-image-build-stage
  COPY --from=neomantra/flatbuffers /usr/local/bin/flatc /usr/local/bin/flatc

  RUN rustup component add rustfmt
  COPY . /app
  WORKDIR /app
  RUN cargo build --release

FROM gcr.io/distroless/cc
  COPY --from=build /app/target/release/lnctl /bin
  ENV PATH /bin:$PATH
  CMD ["/bin/lnctl"]
