/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
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


pub struct PersonsQObject {}

pub struct PersonsEmitter {
    qobject: Arc<AtomicPtr<PersonsQObject>>,
    new_data_ready: fn(*mut PersonsQObject, index: COption<usize>),
}

unsafe impl Send for PersonsEmitter {}

impl PersonsEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> PersonsEmitter {
        PersonsEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const PersonsQObject = null();
        self.qobject.store(n as *mut PersonsQObject, Ordering::SeqCst);
    }
    pub fn new_data_ready(&mut self, item: Option<usize>) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr, item.into());
        }
    }
}

#[derive(Clone)]
pub struct PersonsTree {
    qobject: *mut PersonsQObject,
    layout_about_to_be_changed: fn(*mut PersonsQObject),
    layout_changed: fn(*mut PersonsQObject),
    data_changed: fn(*mut PersonsQObject, usize, usize),
    begin_reset_model: fn(*mut PersonsQObject),
    end_reset_model: fn(*mut PersonsQObject),
    begin_insert_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize),
    end_insert_rows: fn(*mut PersonsQObject),
    begin_move_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize, dest: COption<usize>, usize),
    end_move_rows: fn(*mut PersonsQObject),
    begin_remove_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize),
    end_remove_rows: fn(*mut PersonsQObject),
}

impl PersonsTree {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, index: Option<usize>, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, index.into(), first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, index: Option<usize>, first: usize, last: usize, dest: Option<usize>, destination: usize) {
        (self.begin_move_rows)(self.qobject, index.into(), first, last, dest.into(), destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, index: Option<usize>, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, index.into(), first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait PersonsTrait {
    fn new(emit: PersonsEmitter, model: PersonsTree) -> Self;
    fn emit(&mut self) -> &mut PersonsEmitter;
    fn row_count(&self, _: Option<usize>) -> usize;
    fn can_fetch_more(&self, _: Option<usize>) -> bool {
        false
    }
    fn fetch_more(&mut self, _: Option<usize>) {}
    fn sort(&mut self, _: u8, _: SortOrder) {}
    fn check_row(&self, index: usize, row: usize) -> Option<usize>;
    fn index(&self, item: Option<usize>, row: usize) -> usize;
    fn parent(&self, index: usize) -> Option<usize>;
    fn row(&self, index: usize) -> usize;
    fn user_name(&self, index: usize) -> &str;
    fn set_user_name(&mut self, index: usize, _: String) -> bool;
}

#[no_mangle]
pub extern "C" fn persons_new(
    persons: *mut PersonsQObject,
    persons_new_data_ready: fn(*mut PersonsQObject, index: COption<usize>),
    persons_layout_about_to_be_changed: fn(*mut PersonsQObject),
    persons_layout_changed: fn(*mut PersonsQObject),
    persons_data_changed: fn(*mut PersonsQObject, usize, usize),
    persons_begin_reset_model: fn(*mut PersonsQObject),
    persons_end_reset_model: fn(*mut PersonsQObject),
    persons_begin_insert_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize),
    persons_end_insert_rows: fn(*mut PersonsQObject),
    persons_begin_move_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize, index: COption<usize>, usize),
    persons_end_move_rows: fn(*mut PersonsQObject),
    persons_begin_remove_rows: fn(*mut PersonsQObject, index: COption<usize>, usize, usize),
    persons_end_remove_rows: fn(*mut PersonsQObject),
) -> *mut Persons {
    let persons_emit = PersonsEmitter {
        qobject: Arc::new(AtomicPtr::new(persons)),
        new_data_ready: persons_new_data_ready,
    };
    let model = PersonsTree {
        qobject: persons,
        layout_about_to_be_changed: persons_layout_about_to_be_changed,
        layout_changed: persons_layout_changed,
        data_changed: persons_data_changed,
        begin_reset_model: persons_begin_reset_model,
        end_reset_model: persons_end_reset_model,
        begin_insert_rows: persons_begin_insert_rows,
        end_insert_rows: persons_end_insert_rows,
        begin_move_rows: persons_begin_move_rows,
        end_move_rows: persons_end_move_rows,
        begin_remove_rows: persons_begin_remove_rows,
        end_remove_rows: persons_end_remove_rows,
    };
    let d_persons = Persons::new(persons_emit, model);
    Box::into_raw(Box::new(d_persons))
}

#[no_mangle]
pub unsafe extern "C" fn persons_free(ptr: *mut Persons) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn persons_row_count(
    ptr: *const Persons,
    index: COption<usize>,
) -> c_int {
    to_c_int((&*ptr).row_count(index.into()))
}
#[no_mangle]
pub unsafe extern "C" fn persons_can_fetch_more(
    ptr: *const Persons,
    index: COption<usize>,
) -> bool {
    (&*ptr).can_fetch_more(index.into())
}
#[no_mangle]
pub unsafe extern "C" fn persons_fetch_more(ptr: *mut Persons, index: COption<usize>) {
    (&mut *ptr).fetch_more(index.into())
}
#[no_mangle]
pub unsafe extern "C" fn persons_sort(
    ptr: *mut Persons,
    column: u8,
    order: SortOrder
) {
    (&mut *ptr).sort(column, order)
}
#[no_mangle]
pub unsafe extern "C" fn persons_check_row(
    ptr: *const Persons,
    index: usize,
    row: c_int,
) -> COption<usize> {
    (&*ptr).check_row(index, to_usize(row)).into()
}
#[no_mangle]
pub unsafe extern "C" fn persons_index(
    ptr: *const Persons,
    index: COption<usize>,
    row: c_int,
) -> usize {
    (&*ptr).index(index.into(), to_usize(row))
}
#[no_mangle]
pub unsafe extern "C" fn persons_parent(ptr: *const Persons, index: usize) -> QModelIndex {
    if let Some(parent) = (&*ptr).parent(index) {
        QModelIndex {
            row: to_c_int((&*ptr).row(parent)),
            internal_id: parent,
        }
    } else {
        QModelIndex {
            row: -1,
            internal_id: 0,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn persons_row(ptr: *const Persons, index: usize) -> c_int {
    to_c_int((&*ptr).row(index))
}

#[no_mangle]
pub unsafe extern "C" fn persons_data_user_name(
    ptr: *const Persons, index: usize,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.user_name(index);
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn persons_set_data_user_name(
    ptr: *mut Persons, index: usize,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_user_name(index, v)
}
