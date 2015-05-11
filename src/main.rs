fn fizz(x: i32) {
  for i in (1..x+1) {
    if i % 15 == 0 {
      println!("fizzbuzz");
    } else if i % 5 == 0 {
      println!("buzz");
    } else if i % 3 == 0 {
      println!("fizz");
    } else {
      println!("{}", i);
    }
  }
}

fn main() {
  use std::io;
  println!("how many numbers would you like to fizzbuzz?");
  let mut limit = String::new();
  io::stdin().read_line(&mut limit)
      .ok()
      .expect("failed to get the input");

  let limit: i32 = match limit.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("let's keep it a number you putz");
        return;
      }
    };
  println!("starting fizzbuzz for limit {}", limit);
  fizz(limit);
}
