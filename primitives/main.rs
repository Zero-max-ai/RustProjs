// primitive data types
// int, float, bool, char

// Integer
// rust has signed (+ and -) and unsigned integers ( only+ ) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Unsigned integers.

fn main() {
  let x: i32 = -42;
  let y: u64 = 100;

  println!("Signed Integer: {}", x);
  println!("Unsigned Integer: {}", y);

  // floats [floating point types]
  // f32, f64

  let pi: f64 = 3.14;
  println!("Value of pi: {}", pi);

  // boolean values: true, false

  let is_snowing: bool = true;
  println!("Is it snowing? {}", is_snowing);

  // character type - char
  let letter: char = 'A';
  println!("First letter of the alphaber: {}", letter);


}
