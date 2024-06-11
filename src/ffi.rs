use std::os::raw::c_char;

pub fn to_cstr(s: &str) -> *mut c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}

pub fn to_nullable_cstr(s: Option<&str>) -> *mut c_char {
    if s.is_none() {
        return std::ptr::null_mut();
    }

    std::ffi::CString::new(s.unwrap()).unwrap().into_raw()
}

pub fn release(p: *mut c_char) {
    if !p.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(p);
        }
    }
}

pub fn to_string(s: *const c_char) -> String {
    if s.is_null() {
        return String::new();
    }

    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };

    buffer.to_str().unwrap().to_string()
}

pub fn vec_from_nta(raw: *mut *mut i8) -> Vec<*mut i8> {
    let mut vec = Vec::new();
    if raw.is_null() {
        return vec;
    }

    for x in 0.. {
        unsafe {
            if !(*raw.offset(x)).is_null() {
                vec.push(*raw.offset(x));
            } else {
                break;
            }
        }
    }

    vec
}

pub fn release_nta(raw: *mut *mut i8) {
    if raw.is_null() {
        return;
    }

    for x in 0.. {
        let ptr = unsafe { *raw.offset(x) };
        if ptr.is_null() {
            break;
        }

        lxc_release(ptr);
    }

    lxc_release(raw as _);
}

pub fn lxc_release(raw: *mut i8) {
    unsafe { lxc_sys::free(raw as _) }
}

#[cfg(test)]
mod tests {
    use super::{release, to_cstr, to_nullable_cstr, to_string};

    #[test]
    fn null_test() {
        use super::{lxc_release, release_nta, vec_from_nta};
        use std::ptr::{null, null_mut};

        release(null_mut());
        release_nta(null_mut());
        lxc_release(null_mut());

        assert!(to_string(null()).is_empty());
        assert!(vec_from_nta(null_mut()).is_empty());
    }

    #[test]
    fn cstr_test() {
        let cstr = to_cstr("");
        assert!(!cstr.is_null());
        assert_eq!(unsafe { *cstr }, 0);
        release(cstr);

        let cstr = to_cstr("x");
        assert!(!cstr.is_null());
        assert_eq!(unsafe { *cstr }, b'x' as _);
        assert_eq!(unsafe { *cstr.offset(1) }, 0);
        release(cstr);

        let cstr = to_nullable_cstr(None);
        assert!(cstr.is_null());
        release(cstr);

        let cstr = to_nullable_cstr(Some(""));
        assert!(!cstr.is_null());
        assert_eq!(unsafe { *cstr }, 0);
        release(cstr);

        let cstr = to_nullable_cstr(Some("y"));
        assert!(!cstr.is_null());
        assert_eq!(unsafe { *cstr }, b'y' as _);
        assert_eq!(unsafe { *cstr.offset(1) }, 0);
        release(cstr);
    }

    #[test]
    fn string_test() {
        let cstr = to_cstr("");
        let string = to_string(cstr);
        assert!(string.is_empty());
        release(cstr);

        let cstr = to_cstr("hello");
        let string = to_string(cstr);
        assert_eq!(string, "hello");
        release(cstr);

        let cstr = to_nullable_cstr(None);
        let string = to_string(cstr);
        assert!(string.is_empty());
        release(cstr);

        let cstr = to_nullable_cstr(Some(""));
        let string = to_string(cstr);
        assert!(string.is_empty());
        release(cstr);

        let cstr = to_nullable_cstr(Some("world"));
        let string = to_string(cstr);
        assert_eq!(string, "world");
        release(cstr);
    }
}
