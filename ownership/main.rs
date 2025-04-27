// Ownership, Borrowing and References

// ownership
// ---------
// c, c++ - Memory management control issue
// garbage collector solved this issue, but created a new issue -> slow performance:
// [stopping/resuming the program]

// what is ownership?
// every value has a single owner [every variable has one value, and it is its sole owner].

// ownership rules
// 1- each value in rust has a veriable that's its owner.
// 2- there can be only one owner at a time.
// 3- when the owner goes out of scope, the value will be dropped.



fn main () {
  let s1: String = String::from("Rust");
  let len: usize = calculate_length(&s1);
  println!("len of {} is: {}", s1, len);
  let s2 = s1;
  println!("s2: {}", s2);
  // because s1 is no longer the owner of the string "Rust" which now owned by s2.
  let s3 = String::from("Rust");
  // let len1 = calculate_length(&s3);
  // println!("s3: {}  len: {}", s3, len1);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
