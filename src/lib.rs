extern crate libc;

// copied from libstd/sys/unix/mod.rs and libstd/sys/unix/fd.rs
pub fn cvt(t: ::libc::ssize_t) -> ::std::io::Result<::libc::ssize_t> {
    if t == -1 {
        Err(::std::io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

#[derive(Clone, Copy)]
pub struct FdStream {
    fd : ::libc::c_int,
}

impl FdStream {
    pub fn new(fd : ::libc::c_int) -> FdStream {
        FdStream { fd : fd }
    }
}

impl ::std::io::Read for FdStream {
    fn read(&mut self, buf : &mut [u8]) -> ::std::io::Result<usize> {
        let ret = try!(cvt(unsafe {
            ::libc::read(self.fd,
                         buf.as_mut_ptr() as *mut ::libc::c_void,
                         buf.len() as ::libc::size_t)
        }));
        Ok(ret as usize)
    }
}

impl ::std::io::Write for FdStream {
    fn write(&mut self, buf : &[u8]) -> ::std::io::Result<usize> {
        let ret = try!(cvt(unsafe {
            ::libc::write(self.fd,
                          buf.as_ptr() as *const ::libc::c_void,
                          buf.len() as ::libc::size_t)
        }));
        Ok(ret as usize)
    }
    fn flush(&mut self) -> ::std::io::Result<()> { Ok(()) }
}
