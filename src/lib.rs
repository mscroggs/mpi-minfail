use mpi::environment::Universe;
use mpi::raw::AsRaw;

pub fn test() -> mpi::topology::UserCommunicator {
    let universe: Universe = mpi::initialize().unwrap();
    let comm = universe.world();
    let mut neighbors = Vec::<i32>::new();
    neighbors.push(0);

    unsafe {
        let mut raw_comm = mpi_sys::RSMPI_COMM_NULL;
        mpi_sys::MPI_Dist_graph_create_adjacent(
            comm.as_raw(),
            neighbors.len() as i32,
            neighbors.as_ptr(),
            mpi_sys::RSMPI_UNWEIGHTED,
            neighbors.len() as i32,
            neighbors.as_ptr(),
            mpi_sys::RSMPI_UNWEIGHTED,
            mpi_sys::RSMPI_INFO_NULL,
            0,
            &mut raw_comm,
        );

        mpi::topology::UserCommunicator::from_raw(raw_comm).unwrap()
    }
}
