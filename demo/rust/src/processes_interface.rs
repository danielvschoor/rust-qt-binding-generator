/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
#![allow(unused_imports)]
use libc::{c_int, c_void};
use types::*;
use std::sync::{Arc, Mutex};
use std::ptr::null;

use processes_implementation::*;

pub struct ProcessesQObject {}

#[derive (Clone)]
pub struct ProcessesEmitter {
    qobject: Arc<Mutex<*const ProcessesQObject>>,
    new_data_ready: fn(*const ProcessesQObject, item: usize, valid: bool),
}

unsafe impl Send for ProcessesEmitter {}

impl ProcessesEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn new_data_ready(&self, item: Option<usize>) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
             (self.new_data_ready)(ptr, item.unwrap_or(13), item.is_some());
        }
    }
}

pub struct ProcessesUniformTree {
    qobject: *const ProcessesQObject,
    begin_reset_model: fn(*const ProcessesQObject),
    end_reset_model: fn(*const ProcessesQObject),
    begin_insert_rows: fn(*const ProcessesQObject, item: usize, valid: bool, usize, usize),
    end_insert_rows: fn(*const ProcessesQObject),
    begin_remove_rows: fn(*const ProcessesQObject, item: usize, valid: bool, usize, usize),
    end_remove_rows: fn(*const ProcessesQObject),
}

impl ProcessesUniformTree {
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, item: Option<usize>, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, item.unwrap_or(13), item.is_some(), first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, item: Option<usize>, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, item.unwrap_or(13), item.is_some(), first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ProcessesTrait {
    fn create(emit: ProcessesEmitter, model: ProcessesUniformTree) -> Self;
    fn emit(&self) -> &ProcessesEmitter;
    fn row_count(&self, Option<usize>) -> usize;
    fn can_fetch_more(&self, Option<usize>) -> bool { false }
    fn fetch_more(&mut self, Option<usize>) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn index(&self, item: Option<usize>, row: usize) -> usize;
    fn parent(&self, item: usize) -> Option<usize>;
    fn row(&self, item: usize) -> usize;
    fn cmd(&self, item: usize) -> String;
    fn cpu_percentage(&self, item: usize) -> u8;
    fn cpu_usage(&self, item: usize) -> f32;
    fn memory(&self, item: usize) -> u64;
    fn name(&self, item: usize) -> String;
    fn pid(&self, item: usize) -> u32;
    fn uid(&self, item: usize) -> u32;
}

#[no_mangle]
pub extern "C" fn processes_new(qobject: *const ProcessesQObject,
        new_data_ready: fn(*const ProcessesQObject, item: usize, valid: bool),
        begin_reset_model: fn(*const ProcessesQObject),
        end_reset_model: fn(*const ProcessesQObject),
        begin_insert_rows: fn(*const ProcessesQObject, item: usize, valid: bool,
            usize,
            usize),
        end_insert_rows: fn(*const ProcessesQObject),
        begin_remove_rows: fn(*const ProcessesQObject, item: usize, valid: bool,
            usize,
            usize),
        end_remove_rows: fn(*const ProcessesQObject))
        -> *mut Processes {
    let emit = ProcessesEmitter {
        qobject: Arc::new(Mutex::new(qobject)),
        new_data_ready: new_data_ready,
    };
    let model = ProcessesUniformTree {
        qobject: qobject,
        begin_reset_model: begin_reset_model,
        end_reset_model: end_reset_model,
        begin_insert_rows: begin_insert_rows,
        end_insert_rows: end_insert_rows,
        begin_remove_rows: begin_remove_rows,
        end_remove_rows: end_remove_rows,
    };
    let d = Processes::create(emit, model);
    Box::into_raw(Box::new(d))
}

#[no_mangle]
pub unsafe extern "C" fn processes_free(ptr: *mut Processes) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn processes_row_count(ptr: *const Processes, item: usize, valid: bool) -> c_int {
    if valid {
        (&*ptr).row_count(Some(item)) as c_int
    } else {
        (&*ptr).row_count(None) as c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_can_fetch_more(ptr: *const Processes, item: usize, valid: bool) -> bool {
    if valid {
        (&*ptr).can_fetch_more(Some(item))
    } else {
        (&*ptr).can_fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_fetch_more(ptr: *mut Processes, item: usize, valid: bool) {
    if valid {
        (&mut *ptr).fetch_more(Some(item))
    } else {
        (&mut *ptr).fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_sort(ptr: *mut Processes, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}
#[no_mangle]
pub unsafe extern "C" fn processes_index(ptr: *const Processes, item: usize, valid: bool, row: c_int) -> usize {
    if !valid {
        (&*ptr).index(None, row as usize)
    } else {
        (&*ptr).index(Some(item), row as usize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_parent(ptr: *const Processes, index: usize) -> QModelIndex {
    if let Some(parent) = (&*ptr).parent(index) {
        QModelIndex::create((&*ptr).row(parent) as c_int, parent)
    } else {
        QModelIndex::invalid()
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_row(ptr: *const Processes, item: usize) -> c_int {
    (&*ptr).row(item) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cmd(ptr: *const Processes, item: usize,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).cmd(item);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cpu_percentage(ptr: *const Processes, item: usize) -> u8 {
    (&*ptr).cpu_percentage(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cpu_usage(ptr: *const Processes, item: usize) -> f32 {
    (&*ptr).cpu_usage(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_memory(ptr: *const Processes, item: usize) -> u64 {
    (&*ptr).memory(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_name(ptr: *const Processes, item: usize,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).name(item);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_pid(ptr: *const Processes, item: usize) -> u32 {
    (&*ptr).pid(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_uid(ptr: *const Processes, item: usize) -> u32 {
    (&*ptr).uid(item).into()
}
