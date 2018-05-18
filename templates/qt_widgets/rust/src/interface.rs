/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .into_iter()
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct SimpleQObject {}

#[derive(Clone)]
pub struct SimpleEmitter {
    qobject: Arc<Mutex<*const SimpleQObject>>,
    message_changed: fn(*const SimpleQObject),
}

unsafe impl Send for SimpleEmitter {}

impl SimpleEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn message_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.message_changed)(ptr);
        }
    }
}

pub trait SimpleTrait {
    fn new(emit: SimpleEmitter) -> Self;
    fn emit(&self) -> &SimpleEmitter;
    fn message(&self) -> &str;
    fn set_message(&mut self, value: String);
}

#[no_mangle]
pub extern "C" fn simple_new(
    simple: *mut SimpleQObject,
    message_changed: fn(*const SimpleQObject),
) -> *mut Simple {
    let simple_emit = SimpleEmitter {
        qobject: Arc::new(Mutex::new(simple)),
        message_changed: message_changed,
    };
    let d_simple = Simple::new(simple_emit);
    Box::into_raw(Box::new(d_simple))
}

#[no_mangle]
pub unsafe extern "C" fn simple_free(ptr: *mut Simple) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub extern "C" fn simple_message_get(
    ptr: *const Simple,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = unsafe { &*ptr };
    let v = o.message();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub extern "C" fn simple_message_set(ptr: *mut Simple, v: *const c_ushort, len: c_int) {
    let o = unsafe { &mut *ptr };
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_message(s);
}
