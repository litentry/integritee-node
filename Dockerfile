# global args that are used across multiple stages
ARG PROFILE

# ==========================
# stage 1: build
# https://hub.docker.com/_/rust
# our host is based on bullseye/sid => rust:latest
# ==========================
FROM rust:latest as builder

WORKDIR /integritee
COPY . /integritee

RUN apt-get update && apt-get install -yq clang libclang-dev cmake protobuf-compiler

# install sccache, must before `ARG RUSTC_WRAPPER`
# otherwise the wrapper is set but sccache is not installed
RUN cargo install sccache

ARG BUILD_ARGS
ARG PROFILE
ARG RUSTC_WRAPPER

RUN type sccache && sccache --version
ENV SCCACHE_CACHE_SIZE=10G
ENV SCCACHE_DIR=/root/.cache/sccache
ENV RUSTC_WRAPPER=$RUSTC_WRAPPER

# please note this only works for self-hosted runner (i.e. on the same host)
# CI across different GH-runners won't work well, my understanding is docker only considers
# image layers as "bulid cache", and mounted cache doesn't belong to it and therefore not
# exported/imported with build-push-action
#
# see https://github.com/docker/build-push-action/issues/716
#     https://github.com/moby/buildkit/issues/1512
#     https://github.com/moby/buildkit/issues/1673
RUN --mount=type=cache,target=/root/.cache/sccache cargo build --locked --profile $PROFILE $BUILD_ARGS && sccache --show-stats

# ==========================
# stage 2: packaging
# ==========================
FROM ubuntu:20.04

ARG PROFILE

COPY --from=builder /integritee/target/$PROFILE/integritee-node /usr/local/bin

# unclutter and minimize the attack surface
RUN rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/integritee-node --version

ENTRYPOINT ["/usr/local/bin/integritee-node"]
CMD ["--help"]