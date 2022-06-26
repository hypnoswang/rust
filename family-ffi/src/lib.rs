extern crate family;

use libc::{c_char, c_uint};
use std::ffi::CStr;

pub use family::{Family, Person};

// 我们不能直接返回rust的结构体到C语言，因为Rust的结构体可能包含有rust特有的结构成员，比如String、HashMap
// 我们只能返回一个结构体的指针
#[no_mangle]
pub extern "C" fn create_person(name: *const c_char, age: c_uint) -> *mut Person {
    assert!(!name.is_null());

    let rn = unsafe {
        CStr::from_ptr(name)
            .to_str()
            .expect("Invalid name for person")
    };

    let p = Box::new(Person::new(rn, age));

    Box::into_raw(p)
}

// 因为C语言没有自动内存回收，所以，需要我们手动回收暴漏的裸指针的内存
#[no_mangle]
pub extern "C" fn free_person(p: *mut Person) {
    assert!(!p.is_null());

    // 使用box来回收内存
    unsafe {
        Box::from_raw(p);
    }
}

#[no_mangle]
pub extern "C" fn create_family(name: *const c_char, count: c_uint) -> *mut Family {
    assert!(!name.is_null());

    let fnm = unsafe {
        CStr::from_ptr(name)
            .to_str()
            .expect("Invalid name for person")
    };

    let p = Box::new(Family::new(fnm, count as usize));

    Box::into_raw(p)
}

#[no_mangle]
pub extern "C" fn free_family(f: *mut Family) {
    assert!(!f.is_null());

    // 使用box来回收内存
    unsafe {
        Box::from_raw(f);
    }
}

// p的内存由调用者自己释放，这里使用了p的一个副本
#[no_mangle]
pub extern "C" fn add_person(f: *mut Family, p: *const Person) {
    assert!(!f.is_null());
    assert!(!p.is_null());

    let rf;
    let rp;

    unsafe {
        rf = &mut *f;
        rp = &*p;
    }

    let np = Person::from(rp);

    rf.add_person(np);
}

#[no_mangle]
pub extern "C" fn display_family(f: *const Family) {
    assert!(!f.is_null());

    let rf;

    unsafe {
        rf = &*f;
    }

    rf.display();
}
