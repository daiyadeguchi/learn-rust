fn main() {
  // example of borrowing
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);

  // use mut keyword to make the reference mutable
  let mut s = String::from("heeeeee");
  change(&mut s);
  println!("mutable = {}", s);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
