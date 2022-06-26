use libc::{c_char, c_int};

#[repr(C)]
pub struct Person {
    pub name: *mut c_char,
    pub age: c_int,
}

#[repr(C)]
pub struct Home {
    pub name: *mut c_char,
    pub member_cnt: c_int,
    pub members: *mut *const Person,
}

extern "C" {
    pub fn create_person(n: *const c_char, a: c_int) -> *mut Person;
    pub fn free_person(p: *const Person);
    pub fn display_person(p: *const Person);

    pub fn create_home(n: *const c_char, c: c_int) -> *mut Home;
    pub fn free_home(h: *const Home);
    pub fn add_person(h: *mut Home, p: *const Person);
    pub fn display_home(h: *const Home);
}
