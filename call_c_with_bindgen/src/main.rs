use call_c_with_bindgen::*;
use std::ffi::CString;

fn main() {
    let persons = vec!["Homer", "Marge", "Bart", "Lisa", "Maggie"];

    unsafe {
        let fname = CString::new("Simpons").unwrap();

        let f = create_home(fname.as_ptr(), 10);
        for p in persons {
            let pn = CString::new(p).unwrap();
            let p = create_person(pn.as_ptr(), 12);
            add_person(f, p);
        }

        display_home(f);

        free_home(f);
    }
}
