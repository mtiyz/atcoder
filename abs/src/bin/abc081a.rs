use proconio::input;

fn main() {
  input! {
    str1: String
  }
  let ans = str1.chars().filter(|&c| c == '1').count();
  println!("{}", ans);
}
