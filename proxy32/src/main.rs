#[cfg(target_arch = "x86")]
fn main() {
    println!("Hello, world!");
}

#[cfg(not(target_arch = "x86"))]
fn main() {
    panic!("X86 arch target only!");
}
