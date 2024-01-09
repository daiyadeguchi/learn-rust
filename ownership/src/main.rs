fn main() {
  let s1 = String::from("hello");
  let s2 = s1;

  // cannot use anymore s1 because s1 is moved to s2
  // println!("{}, world!", s1);
  println!("{}, world!", s2);

  // .clone() is there when you need to clone something
  // clone may be a good indication that code might be expensive
  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("s3 = {}, s4 = {}", s3, s4);

  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);

  let s = String::from("hello"); // s into scope
  takes_ownership(s);            // s moves into the function
                                 // so it's no longer valid here
  let n = 5;    // n into scope
  makes_copy(n);// n to function
                // n still is valid, because it's i32

  let s5 = gives_ownership();       // gives_ownership moves its return value to s5
  println!("s5 = {}", s5);
  let s6 = String::from("hello");   // s6 into scope
  let s7 = takes_and_gives_back(s6);// s6 is moved to the fn
  println!("s7 = {}", s7);

  let s8 = String::from("hello");
  let (s9, len) = calculate_length(s8);
  println!("The length of '{}' is {}", s9, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // some_string 'drop'ed and memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // some_integer out of scope.

fn gives_ownership() -> String {
  let some_string = String::from("yours");
  // the last expression returns the value
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}
