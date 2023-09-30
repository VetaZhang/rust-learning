
fn main() {
  let n = 29;
  let mut a: u64 = 1;
  let mut b: u64 = 1;

  
  for i in 1..n {
    if i == 1 {
      println!("{}", a);
    } else if i == 2 {
      println!("{}", b);
    } else {
      let t: u64 = a + b;
      a = b;
      b = t;
      println!("{}", t);
    }
  }
}