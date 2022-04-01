FROM rust:latest
# see https://github.com/neomantra/docker-flatbuffers#usage-as-an-image-build-stage
COPY --from=neomantra/flatbuffers /usr/local/bin/flatc /usr/local/bin/flatc

RUN rustc --version

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN git clone https://github.com/bats-core/bats-core.git \
            && cd bats-core \
            && ./install.sh /usr/local

RUN apt-get update && apt-get install -y \
  curl rsync wget ca-certificates gnupg lsb-release gettext-base

RUN apt-get remove -y docker docker.io runc
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
RUN echo  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null \
   && apt-get update \
   && apt-get install -y docker-ce docker-ce-cli containerd.io

RUN mkdir -p /usr/local/lib/docker/cli-plugins \
  && curl -SL https://github.com/docker/compose/releases/download/v2.0.1/docker-compose-linux-x86_64 -o /usr/local/lib/docker/cli-plugins/docker-compose \
  && chmod +x /usr/local/lib/docker/cli-plugins/docker-compose
