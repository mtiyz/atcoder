use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [u32; n]
  }

  a.sort_by_key(|&x| std::cmp::Reverse(x));

  let mut alice = 0;
  let mut bob = 0;

  for i in 0..n {
    if i % 2 == 0 {
      // Alice
      alice += a[i];
    } else {
      // Bob
      bob += a[i];
    }
  }

  let ans = alice - bob;

  println!("{}", ans);
}
