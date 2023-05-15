#[cfg(feature = "new")]
pub unsafe fn unweighted() -> *mut i32 {
    mpi_sys::RSMPI_UNWEIGHTED()
}

#[cfg(not(feature = "new"))]
pub unsafe fn unweighted() -> *mut i32 {
    mpi_sys::MPI_UNWEIGHTED
}
