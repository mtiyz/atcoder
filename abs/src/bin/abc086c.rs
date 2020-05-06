use proconio::input;

fn main() {
  input! {
    n: u32,
    mut v: [(i32, i32, i32); n]
  }

  // ほぼパクリ

  v.insert(0, (0, 0, 0)); // 初期値の0, 0を追加

  // windowsで2こだけにする
  let is_possible: bool = v[..].windows(2).all(|w| {
    let (t, x, y) = w[0];
    let (next_t, next_x, next_y) = w[1];
    let time = next_t - t;
    let way = (next_x - x).abs() + (next_y - y).abs();
    way <= time && time % 2 == way % 2
  });

  let ans: &str = if is_possible { "Yes" } else { "No" };

  println!("{}", ans);
}
