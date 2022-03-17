FROM rust:latest as builder

WORKDIR /integritee
COPY . /integritee

RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

ARG BUILD_ARGS

RUN cargo build --release $BUILD_ARGS

FROM ubuntu:20.04
LABEL maintainer="litentry-dev"

COPY --from=builder /integritee/target/release/integritee-node /usr/local/bin
RUN chmod +x /usr/local/bin/integritee-node


RUN useradd -m -u 1000 -U -s /bin/sh -d /integritee integritee && \
	mkdir -p /data /integritee/.local/share && \
	chown -R integritee:integritee /data && \
	ln -s /data /integritee/.local/share/integritee-node && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/integritee-node --version

USER integritee
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/integritee-node"]
CMD ["--help"]