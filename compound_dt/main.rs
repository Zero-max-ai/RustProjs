// compound data types
// arrays, tuples, slices and strings (slice string)

// arrays
fn main() {
  let numbers: [i8; 5] = [1,2,3,4,5];
  println!("Number Array: {:?}", numbers);

  let fruits: [&str; 5] = ["apples", "mangoes", "banana", "orange", "kiwi"];
  println!("Fruits Array: {:?}", fruits);

  let human: (String, i8, bool) = ("Alice".to_string(), 30, false);
  println!("Human Tuple: {:?}", human);

  let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
  println!("My Mix Tuple: {:?}", my_mix_tuple);

  // Slices: [1,2,3,4,5]
  let number_slices:&[i8] = &[1,2,3,4,5];
  println!("Number slice: {:?}", number_slices);

  let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
  println!("Animal slice: {:?}", animal_slices);

  let book_slices: &[&String] = &[&"IT".to_string(), &"HarryPotter".to_string()];
  println!("Book Slice: {:?}", book_slices);

  // strings vs string slices (&str)
  // strings [ growable, mutable, owned string type ]
  let mut stone_cold: String = String::from("Hell");
  println!("Stone Cold says: {}", stone_cold);
  stone_cold.push_str("Yeah!");
  println!("Now Stone Cold says: {}", stone_cold);
  
  // b- &str (String Slice)
  let string: String = String::from("Hello, World!");
  let slice: &str = &string;
  println!("Slice Value: {}", slice);
  let slice_part: &str = &string[0..5];
  println!("Slice portion: {}", slice_part);



}
