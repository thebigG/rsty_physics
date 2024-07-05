FROM rust:1.70
RUN apt update &&  \
    apt-get -y install clang && \
    apt-get -y install libclang-dev && \
    apt-get install -y llvm && \
    apt-get install -y sudo \
    libx11-dev \
    libxcursor-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libglu-dev \
    libasound2-dev \
    libpulse-dev \
    libudev-dev \
    libxi-dev \
    libxrandr-dev

RUN useradd -ms /bin/bash -G sudo lgomez
USER lgomez
ARG GODOT_VERSION=4.2.2
WORKDIR /home/lgomez
COPY --chown=lgomez:lgomez . /home/lgomez
RUN rustup component add rustfmt
RUN rustfmt --check /home/lgomez/rsty_physics/src/*.rs
#https://github.com/godotengine/godot/releases/download/4.2.1-stable/Godot_v4.2.1-stable_linux.x86_64.zip
#wget https://downloads.tuxfamily.org/godotengine/${GODOT_VERSION}/Godot_v${GODOT_VERSION}-stable_linux.x86_64.zip
RUN cd /home/lgomez/ && wget https://github.com/godotengine/godot/releases/download/${GODOT_VERSION}-stable/Godot_v${GODOT_VERSION}-stable_linux.x86_64.zip\
    && unzip Godot_v${GODOT_VERSION}-stable_linux.x86_64.zip
#These might be useful for wasm deployments in the future.
#RUN rustup target add wasm32-unknown-unknown
#RUN cargo install trunk
env GODOT4_BIN=/home/lgomez/Godot_v${GODOT_VERSION}-stable_linux.x86_64
#wget https://downloads.tuxfamily.org/godotengine/${GODOT_VERSION}/Godot_v${GODOT_VERSION}-stable_export_templates.tpz
RUN cd /home/lgomez/ && wget https://github.com/godotengine/godot/releases/download/${GODOT_VERSION}-stable/Godot_v${GODOT_VERSION}-stable_export_templates.tpz
RUN mv /home/lgomez/Godot_v${GODOT_VERSION}-stable_export_templates.tpz /home/lgomez/Godot_v${GODOT_VERSION}-stable_export_templates.zip
RUN cd /home/lgomez/ &&  unzip /home/lgomez/Godot_v${GODOT_VERSION}-stable_export_templates.zip
#RUN mv /home/lgomez/templates  /home/lgomez/.local/share/godot/export_templates/${GODOT_VERSION}.beta6

RUN cd /home/lgomez/rsty_physics && cargo clean && cargo build && cargo test
