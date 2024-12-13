use std::{
    ffi::{CStr, CString},
    net::SocketAddr,
    os::raw::c_char,
    ptr,
};

use libc::{addrinfo, gai_strerror, sockaddr, AF_INET, AF_INET6, IPPROTO_TCP};

unsafe fn getaddrinfo(node: &str) -> (i32, *mut *mut addrinfo) {
    // res0 because its a linked list
    let mut res0: *mut addrinfo = ptr::null_mut();
    let node = match CString::new(node) {
        Ok(n) => n,
        Err(_) => {
            return (1, ptr::null_mut());
        }
    };

    let c_node = node.as_ptr();
    let error = unsafe { libc::getaddrinfo(c_node, c"http".as_ptr(), ptr::null(), &mut res0) };

    (error, &mut res0)
}

fn getsocketaddr(addrinfo: *const addrinfo) -> SocketAddr {
    let current = addrinfo;

    // while !current.is_null() {
    //     match (*current).ai_family {
    //         AF_INET => {}
    //         AF_INET6 => {}
    //         _ => {}
    //     }
    // }
}

pub fn fetch(url: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getaddrinfo_works() {
        unsafe {
            let (err, addrinfo) = getaddrinfo("google.com");
            if err != 0 {
                let msg = gai_strerror(err);
                let c_str = CStr::from_ptr(msg);
                println!("Error: {c_str:?}");
            }
            assert_eq!(err, 0);
            let first = **addrinfo;
            let addr = *first.ai_addr;
        }
    }
}
