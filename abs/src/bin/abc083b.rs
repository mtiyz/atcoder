use proconio::input;

fn main() {
  input! {
    n: u32,
    a: u32,
    b: u32
  }

  // ほぼパクリ

  let ans: u32 = (1..n + 1)
    .filter(|x| {
      let s: u32 = x
        .to_string()
        .chars()
        .map(|c| c as u32 - 48)
        .sum();
      let res = if a < s + 1 && s < b + 1 { true } else { false };
      res
    })
    .sum();
  println!("{}", ans);
}
