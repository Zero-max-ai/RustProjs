// References and Borrowing
// Safety and Performace
// Borrowing and references are powerful concepts

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.
// Create Refernce by add '&'
// -I- Immutable Reference

/*
fn main() {
  let mut x: i32 = 5;
  let _r: &mut i32 = &mut x;
  *_r += 1;
  *_r -= 3;

  println!("Value of x: {}", x);
}
*/

fn main() {
  let mut user: BankAccount = BankAccount{
    owner: "Alice".to_string(),
    balance: 160.5
  };

  user.check_balance();
  user.withdraw(53.1);
  user.check_balance();


}

struct BankAccount {
  owner: String,
  balance: f64,
}

impl BankAccount {
  fn withdraw(&mut self, amount: f64) {
    println!("Withdrawing: {}, from account owned by: {}", amount, self.owner);
    self.balance -= amount;
  }

  fn check_balance(&self) {
    println!("The balance is {}, of owner: {}", self.balance, self.owner);
  }
}
