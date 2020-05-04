use proconio::input;

fn main() {
  input! {
    num1: u32,
    num2: u32,
    num3: u32,
    s: String
  }
  let res = num1 + num2 + num3;
  println!("{} {}", res, s);
}
