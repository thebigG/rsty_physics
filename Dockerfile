FROM rust:1.64
RUN apt update &&  \
    apt-get -y install clang && \
    apt-get -y install libclang-dev && \
    apt-get install -y llvm && \
    apt-get install -y sudo

RUN useradd -ms /bin/bash -G sudo lgomez
USER lgomez
WORKDIR /home/lgomez
COPY --chown=lgomez:lgomez . /home/lgomez
RUN rustup component add rustfmt
RUN rustfmt --check /home/lgomez/rsty_physics/src/*.rs
RUN cd /home/lgomez/ && wget https://downloads.tuxfamily.org/godotengine/4.0/beta6/Godot_v4.0-beta6_linux.x86_64.zip  \
    && unzip Godot_v4.0-beta6_linux.x86_64.zip
#These might be useful for wasm deployments in the future.
#RUN rustup target add wasm32-unknown-unknown
#RUN cargo install trunk
RUN cd /home/lgomez/rsty_physics && cargo clean && GODOT4_BIN=/home/lgomez/Godot_v4.0-beta6_linux.x86_64 cargo build
