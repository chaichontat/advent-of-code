FROM ubuntu:latest

ARG DEBIAN_FRONTEND=noninteractive
# persist at runtime
ENV TZ="America/New York"

RUN apt-get update && apt-get -y upgrade
RUN apt-get -y install build-essential git-all curl

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH=/root/.cargo/bin:$PATH

ARG CACHEBUST=1
RUN cd /root && git clone https://github.com/chaichontat/advent-of-code

WORKDIR /root/advent-of-code/2019/rust
ENTRYPOINT ["sh", "-c", "cargo test"]