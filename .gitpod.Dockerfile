FROM gitpod/workspace-full-vnc

RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.14.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN sudo mv ./trunk /usr/bin/

RUN rustup default nightly
