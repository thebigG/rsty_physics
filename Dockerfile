FROM rust:1.64
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
WORKDIR /home/lgomez
COPY --chown=lgomez:lgomez . /home/lgomez
RUN rustup component add rustfmt
RUN rustfmt --check /home/lgomez/rsty_physics/src/*.rs
RUN cd /home/lgomez/ && wget https://downloads.tuxfamily.org/godotengine/4.0/beta6/Godot_v4.0-beta6_linux.x86_64.zip  \
    && unzip Godot_v4.0-beta6_linux.x86_64.zip
#These might be useful for wasm deployments in the future.
#RUN rustup target add wasm32-unknown-unknown
#RUN cargo install trunk
env GODOT4_BIN=/home/lgomez/Godot_v4.0-beta6_linux.x86_64
RUN cd /home/lgomez/ && wget https://downloads.tuxfamily.org/godotengine/4.0/beta6/Godot_v4.0-beta6_export_templates.tpz
RUN mv /home/lgomez/Godot_v4.0-beta6_export_templates.tpz /home/lgomez/Godot_v4.0-beta6_export_templates.zip
RUN cd /home/lgomez/ &&  unzip /home/lgomez/Godot_v4.0-beta6_export_templates.zip
#RUN mv /home/lgomez/templates  /home/lgomez/.local/share/godot/export_templates/4.0.beta6

RUN cd /home/lgomez/rsty_physics && cargo clean && cargo build && cargo test
