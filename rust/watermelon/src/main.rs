use std::io;

fn compute(w: i32) -> &'static str {
  if w < 4 {
    return "NO";
  }

  if w % 2 != 0 {
    return "NO";
  }

  return "YES";
}

fn main() {
  let mut buffer = String::new();

  let stdin = io::stdin();
  let _ = stdin.read_line(&mut buffer);

  let w= buffer.trim().parse::<i32>().unwrap();

  println!("{}", compute(w));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_compute() {
    let result = compute(8);
    assert_eq!(result, "YES");
  }
}
