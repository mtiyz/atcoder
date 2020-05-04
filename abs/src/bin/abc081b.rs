use proconio::input;

fn main() {
  input! {
    n: u32
  }
  let ans: u32 = (0..n)
    .map(|_| {
      input! {
        a: u32
      }
      a.trailing_zeros()
    })
    .min()
    .unwrap();
  println!("{}", ans);
}
