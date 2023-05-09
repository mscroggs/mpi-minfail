FROM ubuntu:22.04 as rust-test

RUN apt-get update
RUN apt-get -y install libopenmpi-dev libopenblas-dev liblapack-dev libclang-dev \
    git curl pkg-config libssl-dev build-essential
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN git clone https://github.com/mscroggs/mpi-minfail.git
RUN cd rlst && RUST_BACKTRACE=full $HOME/.cargo/bin/cargo build --features "strict,mpi"
