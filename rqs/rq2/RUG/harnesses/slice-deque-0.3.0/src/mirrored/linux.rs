//! Non-racy linux-specific mirrored memory allocation.
use libc::{
    c_char, c_int, c_long, c_uint, c_void, close, ftruncate, mkstemp, mmap, munmap,
    off_t, size_t, syscall, sysconf, SYS_memfd_create, ENOSYS, MAP_FAILED, MAP_FIXED,
    MAP_SHARED, PROT_READ, PROT_WRITE, _SC_PAGESIZE,
};
#[cfg(target_os = "android")]
use libc::__errno;
#[cfg(not(target_os = "android"))]
use libc::__errno_location;
use super::{ptr, AllocError};
/// [`memfd_create`] - create an anonymous file
///
/// [`memfd_create`]: http://man7.org/linux/man-pages/man2/memfd_create.2.html
fn memfd_create(name: *const c_char, flags: c_uint) -> c_long {
    unsafe { syscall(SYS_memfd_create, name, flags) }
}
/// Returns the size of a memory allocation unit.
///
/// In Linux-like systems this equals the page-size.
pub fn allocation_granularity() -> usize {
    unsafe { sysconf(_SC_PAGESIZE) as usize }
}
/// Reads `errno`.
fn errno() -> c_int {
    #[cfg(not(target_os = "android"))] unsafe { *__errno_location() }
    #[cfg(target_os = "android")] unsafe { *__errno() }
}
/// Allocates an uninitialzied buffer that holds `size` bytes, where
/// the bytes in range `[0, size / 2)` are mirrored into the bytes in
/// range `[size / 2, size)`.
///
/// On Linux the algorithm is as follows:
///
/// * 1. Allocate a memory-mapped file containing `size / 2` bytes.
/// * 2. Map the file into `size` bytes of virtual memory.
/// * 3. Map the file into the last `size / 2` bytes of the virtual memory
/// region      obtained in step 2.
///
/// This algorithm doesn't have any races.
///
/// # Panics
///
/// If `size` is zero or `size / 2` is not a multiple of the
/// allocation granularity.
pub fn allocate_mirrored(size: usize) -> Result<*mut u8, AllocError> {
    unsafe {
        let half_size = size / 2;
        assert!(size != 0);
        assert!(half_size % allocation_granularity() == 0);
        let mut fname = *b"/tmp/slice_deque_fileXXXXXX\0";
        let mut fd: c_long = memfd_create(fname.as_mut_ptr() as *mut c_char, 0);
        if fd == -1 && errno() == ENOSYS {
            fd = c_long::from(mkstemp(fname.as_mut_ptr() as *mut c_char));
        }
        if fd == -1 {
            print_error("memfd_create failed");
            return Err(AllocError::Other);
        }
        let fd = fd as c_int;
        if ftruncate(fd, half_size as off_t) == -1 {
            print_error("ftruncate failed");
            if close(fd) == -1 {
                print_error("@ftruncate: close failed");
            }
            return Err(AllocError::Oom);
        }
        let ptr = mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        if ptr == MAP_FAILED {
            print_error("@first: mmap failed");
            if close(fd) == -1 {
                print_error("@first: close failed");
            }
            return Err(AllocError::Oom);
        }
        let ptr2 = mmap(
            (ptr as *mut u8).offset(half_size as isize) as *mut c_void,
            half_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_FIXED,
            fd,
            0,
        );
        if ptr2 == MAP_FAILED {
            print_error("@second: mmap failed");
            if munmap(ptr, size as size_t) == -1 {
                print_error("@second: munmap failed");
            }
            if close(fd) == -1 {
                print_error("@second: close failed");
            }
            return Err(AllocError::Other);
        }
        if close(fd) == -1 {
            print_error("@success: close failed");
        }
        Ok(ptr as *mut u8)
    }
}
/// Deallocates the mirrored memory region at `ptr` of `size` bytes.
///
/// # Unsafe
///
/// `ptr` must have been obtained from a call to `allocate_mirrored(size)`,
/// otherwise the behavior is undefined.
///
/// # Panics
///
/// If `size` is zero or `size / 2` is not a multiple of the
/// allocation granularity, or `ptr` is null.
pub unsafe fn deallocate_mirrored(ptr: *mut u8, size: usize) {
    assert!(! ptr.is_null());
    assert!(size != 0);
    assert!(size % allocation_granularity() == 0);
    if munmap(ptr as *mut c_void, size as size_t) == -1 {
        print_error("deallocate munmap failed");
    }
}
/// Prints last os error at `location`.
#[cfg(all(debug_assertions, feature = "use_std"))]
fn print_error(location: &str) {
    eprintln!("Error at {}: {}", location, ::std::io::Error::last_os_error());
}
/// Prints last os error at `location`.
#[cfg(not(all(debug_assertions, feature = "use_std")))]
fn print_error(_location: &str) {}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let name_str = CString::new(rug_fuzz_0).expect(rug_fuzz_1);
        let name_ptr = name_str.as_ptr() as *const i8;
        let flags: u32 = rug_fuzz_2;
        crate::mirrored::linux::memfd_create(name_ptr, flags);
             }
});    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::mirrored::linux::allocation_granularity;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_18_rrrruuuugggg_test_rug = 0;
        allocation_granularity();
        let _rug_ed_tests_rug_18_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use std::os::raw::c_int;
    #[test]
    fn test_errno() {
        let _rug_st_tests_rug_19_rrrruuuugggg_test_errno = 0;
        let result = crate::mirrored::linux::errno();
        let _rug_ed_tests_rug_19_rrrruuuugggg_test_errno = 0;
    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use std::os::raw::{c_long, c_char, c_int};
    use libc::{memfd_create, mkstemp, ftruncate, close, mmap, munmap};
    use libc::{PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_FIXED};
    use std::ptr;
    use std::mem;
    use std::io::{Error, ErrorKind};
    #[test]
    fn test_allocate_mirrored() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let size: usize = rug_fuzz_0;
        let result = allocate_mirrored(size);
        match result {
            Ok(ptr) => {
                unsafe {
                    munmap(ptr as *mut libc::c_void, size);
                }
            }
            Err(err) => {
                panic!("Allocation failed: {:?}", err);
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::mirrored::linux::{deallocate_mirrored, allocation_granularity};
    use std::ptr;
    use libc::{munmap, c_void, size_t};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: *mut u8 = ptr::null_mut();
        let p1: usize = rug_fuzz_0;
        unsafe {
            deallocate_mirrored(p0, p1);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    #[test]
    fn test_print_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        crate::mirrored::linux::print_error(&p0);
             }
});    }
}
