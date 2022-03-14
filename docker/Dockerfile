FROM rust:latest as builder
LABEL maintainer="dev@litentry.com"

WORKDIR /integritee


# RUN mv /usr/share/ca* /tmp && \
# 	rm -rf /usr/share/*  && \
# 	mv /tmp/ca-certificates /usr/share/ && \
# 	useradd -m -u 1000 -U -s /bin/sh -d /integritee integritee && \
# 	mkdir -p /integritee/.local/share/integritee-node && \
# 	chown -R integritee:integritee /integritee/.local && \
# 	ln -s /integritee/.local/share/integritee-node /data

COPY . /integritee

RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

ARG BUILD_ARGS

RUN cargo build --release

COPY targer/release/integritee-node /usr/local/bin
RUN chmod +x /usr/local/bin/integritee-node

# checks
RUN ldd /usr/local/bin/integritee-node && \
	/usr/local/bin/integritee-node --version

# Shrinking
#RUN rm -rf /usr/lib/python* && \
#	rm -rf /usr/bin /usr/sbin /usr/share/man

#USER integritee
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/integritee-node"]