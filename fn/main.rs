// entry point
// any functions / variables should be written in snake case
// snake case: hello_world
// kebab case: hello-world
fn main() {
  println!("Hello World!");
  hello();
  tell_height(178);
  human_id("Akshat", 22, 178.3);
  let x: i32 = {
    let price: i32 = 5;
    let qty: i32 = 10;
    price * qty
  };
  println!("Result is: {}", x);
  let y = add(62, 64);
  println!("62 + 64: {}", y);

  println!("You're bmi is: {:.2}",bmi_cal(66.4, 1.5));
}

fn hello() {
  println!("Hello Rust");
}

// you can insert input values
fn tell_height(height: u32) {
  println!("My height is: {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
  println!("My name is {}, I am {} years old, and my height is: {} cm.", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}

// expression and statements
// expression: anything that returns a value.
// statement: anything that does not return a value.

// BMI = height(kg)/height(m)^2

fn bmi_cal(weight_kg: f64, height_m: f64) -> f64 {
  weight_kg / (height_m * height_m)
}
