FROM gitpod/workspace-full

RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.14.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN sudo mv ./trunk /usr/bin/

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
