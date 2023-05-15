use mpi::environment::Universe;
use mpi::traits::{AsRaw, Communicator};
use mpi_minfail::unweighted;
use mpi_sys;
use std::os::raw::c_void;

fn main() {
    let universe: Universe = mpi::initialize().unwrap();
    let comm = universe.world();

    let neighbors = vec![if comm.rank() == 0 { 1 } else { 0 }];

    let neighbor_comm = unsafe {
        let mut raw_comm = mpi_sys::RSMPI_COMM_NULL;
        mpi_sys::MPI_Dist_graph_create_adjacent(
            comm.as_raw(),
            neighbors.len() as i32,
            neighbors.as_ptr(),
            unweighted(),
            neighbors.len() as i32,
            neighbors.as_ptr(),
            unweighted(),
            mpi_sys::RSMPI_INFO_NULL,
            0,
            &mut raw_comm,
        );

        mpi::topology::UserCommunicator::from_raw(raw_comm).unwrap()
    };

    let mut result = vec![2];
    let send_me = vec![if comm.rank() == 0 { 3 } else { 5 }];

    unsafe {
        mpi_sys::MPI_Neighbor_alltoallv(
            send_me.as_ptr() as *const c_void,
            vec![1].as_ptr(),
            vec![0].as_ptr(),
            mpi_sys::RSMPI_UINT64_T,
            result.as_mut_ptr() as *mut c_void,
            vec![1].as_ptr(),
            vec![0].as_ptr(),
            mpi_sys::RSMPI_UINT64_T,
            neighbor_comm.as_raw(),
        );
    }
    if comm.rank() == 0 {
        assert_eq!(result[0], 5);
    } else {
        assert_eq!(result[0], 3);
    }
}
