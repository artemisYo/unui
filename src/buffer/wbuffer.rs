use std::os::fd::OwnedFd;
use wayland_client::protocol::{
    wl_shm::WlShm,
    wl_shm_pool::WlShmPool
};

use super::util::*;

pub struct ShmVec {
    fd: OwnedFd,
    segment: MMapSegment,
    capacity: usize,
    length: usize,
}
impl ShmVec {
    // how in the everliving fuck do we deal with queuehandles?
	pub fn with_size(width: usize, height: usize, q: ()) -> Self {
    	let cap = width * height;
    	let mut fd = create_temp_file(None)
        	.expect("Could not create temp file!");
    	truncate_file(&mut fd, cap as i64)
        	.expect("Could not set length of temp file!");
    	let segment = map_file(&mut fd, cap)
        	.expect("Could not map temp file!");
    	let pool = WlShm::create_pool(&fd, cap as i32, q, ());
    	let wl_buffer;
		todo!();
    	Self {
			fd, segment,
			capacity: cap,
			length: 0
    	}
	}
}
