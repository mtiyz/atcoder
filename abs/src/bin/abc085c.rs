use proconio::input;

fn main() {
  input! {
    a: usize,
    y: usize
  }

  let mut ans1: i32 = -1;
  let mut ans2: i32 = -1;
  let mut ans3: i32 = -1;

  'outer: for i in 0..a + 1 {
    for j in 0..a - i + 1 {
      let k = a - i - j;
      if i * 10000 + j * 5000 + k * 1000 == y {
        ans1 = i as i32;
        ans2 = j as i32;
        ans3 = k as i32;
        break 'outer;
      }
    }
  }

  println!("{} {} {}", ans1, ans2, ans3);
}
