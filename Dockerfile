FROM ubuntu:22.04 as rust-test-openmpi

RUN apt-get update
RUN apt-get -y install libopenmpi-dev libopenblas-dev liblapack-dev libclang-dev \
    git curl pkg-config libssl-dev build-essential
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN git clone https://github.com/mscroggs/mpi-minfail.git
RUN cd mpi-minfail && git clone https://github.com/mscroggs/rsmpi.git && cd rsmpi && git checkout mscroggs/MPI_UNWEIGHTED
RUN cd mpi-minfail && RUST_BACKTRACE=full $HOME/.cargo/bin/cargo build

FROM ubuntu:22.04 as rust-test-mpich

RUN apt-get update
RUN apt-get -y install libmpich-dev libopenblas-dev liblapack-dev libclang-dev \
    git curl pkg-config libssl-dev build-essential
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN git clone https://github.com/mscroggs/mpi-minfail.git
RUN cd mpi-minfail && git clone https://github.com/mscroggs/rsmpi.git && cd rsmpi && git checkout mscroggs/MPI_UNWEIGHTED
RUN cd mpi-minfail && RUST_BACKTRACE=full $HOME/.cargo/bin/cargo build
