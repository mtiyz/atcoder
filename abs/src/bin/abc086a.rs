use proconio::input;

fn main() {
  input! {
    num1: u32,
    num2: u32
  }
  let product = num1 * num2;
  let ans: &str = if (product % 2) == 0 { "Even" } else { "Odd" };
  println!("{}", ans);
}
