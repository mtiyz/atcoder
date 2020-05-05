use proconio::input;

fn main() {
  input! {
    mut s: String
  }

  let patterns = ["eraser", "erase", "dreamer", "dream"];

  let ans: &str;

  loop {
    let i = s.len();
    for j in 0..4 {
      s = s.replace(patterns[j], "");
    }
    if i == s.len() {
      ans = "NO";
      break;
    } else if s.len() == 0 {
      ans = "YES";
      break;
    }
  }

  println!("{}", ans)
}
