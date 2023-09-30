// use std::collections::HashSet;

fn main() {
  let mut t = Vec::from([
    Vec::from([1]),
    // Vec::from([2,4,8,1053]),
    // Vec::from([13,3,6,7]),
    // Vec::from([15,14,12,16]),
    // Vec::from(['.','.','2','.','7','.','.','.','.']),
    // Vec::from(['.','1','5','.','.','.','.','.','.']),
    // Vec::from(['.','.','.','.','.','2','.','.','.']),
    // Vec::from(['.','2','.','9','.','.','.','.','.']),
    // Vec::from(['.','.','4','.','.','.','.','.','.'])
  ]);

  Solution::reverse_string(&mut t);
  println!("{:?}", t);
}

struct Solution {
  //
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let p1 = 0;
        let p2 = 0;
    }
}