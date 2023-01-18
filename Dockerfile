FROM ubuntu

RUN apt-get update
RUN apt-get -y upgrade

RUN apt-get install -y curl build-essential

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.toml Cargo.lock /opt/
COPY src /opt/src

WORKDIR /opt
RUN cargo build --release

ENTRYPOINT cargo run --release