use libc;
use std::os::fd::OwnedFd;

#[derive(Debug)]
pub enum MemFdError {
    NameError,
    ArgumentError,
    FdLimitReached,
    OutOfMem,
    PermError,
}
pub fn create_temp_file(name: Option<&str>) -> Result<OwnedFd, MemFdError> {
    use std::os::fd::FromRawFd;
    let c_name = std::ffi::CString::new(name.unwrap_or("placeholder"))
        .map_err(|_| MemFdError::NameError)?;
    let raw_fd = unsafe { libc::memfd_create(c_name.as_ptr(), 0) };
    if raw_fd == -1 {
        let errno = unsafe { *libc::__errno_location() };
        match errno {
            libc::EPERM => Err(MemFdError::PermError)?,
            libc::ENOMEM => Err(MemFdError::OutOfMem)?,
            libc::EFAULT => Err(MemFdError::NameError)?,
            libc::EINVAL => Err(MemFdError::ArgumentError)?,
            libc::ENFILE => Err(MemFdError::FdLimitReached)?,
            libc::EMFILE => Err(MemFdError::FdLimitReached)?,
            _ => unreachable!(),
        }
    }
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

#[derive(Debug)]
pub enum FTruncError {
    Interrupted,
    LengthError,
    IoError,
    BadFd,
    BadLengthOrBadFd, // WHY UNIX WHY
}
// the file is only taken as mut to show that it is modified (truncated)
// it is however technically not needed
pub fn truncate_file(file: &mut OwnedFd, length: i64) -> Result<(), FTruncError> {
    use std::os::fd::AsRawFd;
    let raw_fd = file.as_raw_fd();
    let failed = unsafe { libc::ftruncate(raw_fd, length) };
    if failed == -1 {
        let errno = unsafe { *libc::__errno_location() };
        match errno {
            libc::EINTR => Err(FTruncError::Interrupted)?,
            libc::EIO  => Err(FTruncError::IoError)?,
            libc::EBADF => Err(FTruncError::BadFd)?,
            libc::EINVAL => Err(FTruncError::BadLengthOrBadFd)?,
            libc::EFBIG => Err(FTruncError::LengthError)?,
            _ => unreachable!(),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum MMapError {
    ProtectionError,
    LimitError,
    BadFd,
    MappingConflict,
    ArgumentError,
    FsError,
    PermError,
}
// needs to be a borrow, such that rust's allocator
// does not spring into action and tries to deallocate it
pub struct MMapSegment(&'static mut [u8]);
impl Drop for MMapSegment {
    fn drop(&mut self) {
        let ptr = self.0.as_mut_ptr() as *mut libc::c_void;
        unsafe { libc::munmap(ptr, self.0.len()) };
    }
}
// while we do take a &mut here, the lifetime of the fd doesn't matter
// as on linux the mmaped region is fine even if the fd is closed
pub fn map_file(file: &mut OwnedFd, size: usize) -> Result<MMapSegment, MMapError> {
    use std::os::fd::AsRawFd;
    type VoidPtr = *mut libc::c_void;
    const PROT: i32 = libc::PROT_READ | libc::PROT_WRITE;
    let raw_fd = file.as_raw_fd();
    let addr = unsafe { libc::mmap(
        0 as VoidPtr,
        size,
        PROT,
        libc::MAP_SHARED,
        raw_fd,
        0,
    ) };
    if addr == usize::MAX as VoidPtr {
        let errno = unsafe { *libc::__errno_location() };
        match errno {
            libc::EACCES => Err(MMapError::ProtectionError)?,
            libc::EAGAIN => Err(MMapError::LimitError)?,
            libc::EBADF => Err(MMapError::BadFd)?,
            libc::EEXIST => Err(MMapError::MappingConflict)?,
            libc::EINVAL => Err(MMapError::ArgumentError)?,
            libc::ENFILE => Err(MMapError::LimitError)?,
            libc::ENODEV => Err(MMapError::FsError)?,
            libc::ENOMEM => Err(MMapError::LimitError)?,
            libc::EOVERFLOW => Err(MMapError::LimitError)?,
            libc::EPERM => Err(MMapError::PermError)?,
            libc::ETXTBSY => Err(MMapError::BadFd)?,
            _ => unreachable!(),
        }
    }
    Ok(MMapSegment(unsafe { std::slice::from_raw_parts_mut(addr as *mut u8, size) }))
}
