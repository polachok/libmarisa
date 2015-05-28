extern crate libmarisa_sys as ffi;
extern crate libc;
use libc::{c_int};
use std::ffi::CString;
use std::ops::Drop;
use std::path::Path;

pub struct KeySet {
    keyset: *const ffi::KeySet
}

pub struct Trie {
    trie: *const ffi::Trie,
}

impl KeySet {
    pub fn new() -> KeySet {
        unsafe {
            KeySet { keyset: ffi::keyset_create() }
        }
    }

    pub fn push(&mut self, str: &str, val: i32) {
        unsafe {
            let len = str.len();
            ffi::keyset_push(self.keyset, CString::new(str).unwrap().as_ptr(), len as c_int, val as c_int);
        }
    }

    pub fn num_keys(&self) -> usize {
        unsafe {
            ffi::keyset_num_keys(self.keyset) as usize
        }
    }
}

impl Drop for KeySet {
    fn drop(&mut self) {
        unsafe {
            ffi::keyset_destroy(self.keyset);
        }
    }
}

impl Trie {
    pub fn new() -> Trie {
        unsafe {
            Trie { trie: ffi::trie_create() }
        }
    }

    pub fn build(&mut self, keyset: &KeySet) {
        unsafe {
            ffi::trie_build(self.trie, keyset.keyset, 0);
        }
    }

    pub fn save(&self, path: &Path) {
        unsafe {
            ffi::trie_save(self.trie, CString::new(path.to_str().unwrap()).unwrap().as_ptr());
        }
    }

    pub fn load(&self, path: &Path) {
        unsafe {
            ffi::trie_mmap(self.trie, CString::new(path.to_str().unwrap()).unwrap().as_ptr());
        }
    }
}
