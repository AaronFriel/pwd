
use std::ffi::{CStr, CString};

use libc::{getuid, c_char, getpwnam, getpwuid, passwd};

use errors::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Passwd {
    pub name: String,
    pub passwd: Option<String>,
    pub uid: u32,
    pub gid: u32,
    pub gecos: Option<String>,
    pub dir: String,
    pub shell: String,
}

fn cstr_to_string(cstr: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(cstr).to_string_lossy().into_owned()
    }
}


impl Passwd {
    fn from_unsafe(pwd: *mut passwd) -> Passwd {
        // take ownership? this will panic if null...so don't do that?
        let pwd = unsafe { *pwd };
        let password = if pwd.pw_passwd.is_null() {
            None
        } else {
            Some(cstr_to_string(pwd.pw_passwd))
        };

        let gecos = if pwd.pw_gecos.is_null() {
            None
        } else {
            Some(cstr_to_string(pwd.pw_gecos))
        };

        Passwd {
            name: cstr_to_string(pwd.pw_name),
            passwd: password,
            uid: pwd.pw_uid as u32,
            gid: pwd.pw_gid as u32,
            gecos: gecos,
            dir: cstr_to_string(pwd.pw_dir),
            shell: cstr_to_string(pwd.pw_shell),
        }
    }

    pub fn from_name(name: &str) -> Result<Option<Passwd>> {
        let cname = CString::new(name)
                            .chain_err(|| format!("Could not make CString out of '{}'", name))?;
        let ptr = cname.as_ptr();
        let pwd = unsafe {
            getpwnam(ptr)
        };
        if pwd.is_null() {
            Ok(None)
        } else {
            Ok(Some(Passwd::from_unsafe(pwd)))
        }
    }

    pub fn from_u32(uid: u32) -> Option<Passwd> {
        let pwd = unsafe {
            getpwuid(uid)
        };
        if pwd.is_null() {
            None
        } else {
            Some(Passwd::from_unsafe(pwd))
        }
    }

    pub fn current_user() -> Option<Passwd> {
        let uid = unsafe { getuid() };
        Passwd::from_u32(uid as u32)
    }
}

