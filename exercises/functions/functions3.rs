// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    // 函数入参必须不为空
    call_me(2);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
