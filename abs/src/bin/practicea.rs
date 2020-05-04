use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let stdin = stdin();
  let stdin = stdin.lock();
  let token: String = stdin
    .bytes()
    .map(|c| c.expect("failed to read char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  token.parse().ok().expect("failed to parse token")
}

fn main() {
  let first_int: u32 = read();
  let second_int: u32 = read();
  let third_int: u32 = read();
  let s: String = read();
  let res = first_int + second_int + third_int;
  println!("{} {}", res, s);
}
