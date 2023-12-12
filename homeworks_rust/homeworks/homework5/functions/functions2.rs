// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)



fn main() {
  let num = 3;
  call_this(num);
}

fn call_this(num: u32) {
  for i in 0..num {
      println!("Loop! number {}", i + 1);
  }
}
