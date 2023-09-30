fn main() {
  let list: Vec<usize> = vec![21, 24, 53, 12, 9, 72, 32];

  mid(&list);
}

fn mid(v: &Vec<usize>) {
  let mut l = Vec::from(&v[..]);
  let len = v.len() as usize;
  let half_index: usize = len / 2;

  if len == 0 {
    println!("Empty array has not middle number");
  } else if len == 1 {
    println!("{}", l[0]);
  } else {
    for i in 0..half_index {
      let mut min = i;
      for j in i..len {
        if &l[min] > &l[j] {
          min = j;
        }
      }
  
      if min != i {
        let t = l[min];
        l[min] = l[i];
        l[i] = t;
      }
    }
  
    if len % 2 == 0 {
      println!("{}", (l[half_index] + l[half_index - 1]) / 2);
    } else {
      println!("{}", l[half_index]);
    }
  }
}