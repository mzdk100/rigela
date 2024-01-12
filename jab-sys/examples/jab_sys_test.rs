use jab_sys;

fn main() {
    unsafe {
        jab_sys::initializeAccessBridge();
    }

    println!("Done!");
}
