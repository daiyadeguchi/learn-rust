fn main() {
  let s = String::from("hello world");
  // 0 can be dropped like [..5] if it's 0
  // let hello = &s[0..5]
  // 11 the same if it's the last byte like [6..]
  // let world = &s[6..11]

  let word = first_word(&s);
  println!("The first word is: {}", word);

}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}
