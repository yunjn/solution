use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;

#[allow(unused)]
pub fn valid_utf8(data: Vec<i32>) -> bool {
    data.iter().fold(0, |bit_idx, curr_num| match bit_idx {
        0 => {
            if curr_num >> 5 == 0b110 {
                1
            } else if curr_num >> 4 == 0b1110 {
                2
            } else if curr_num >> 3 == 0b11110 {
                3
            } else if curr_num >> 7 != 0 {
                i32::MAX
            } else {
                bit_idx
            }
        }
        _ => {
            if curr_num >> 6 != 0b10 {
                return i32::MAX;
            }
            bit_idx - 1
        }
    }) == 0
}

#[allow(unused)]
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    if list1.len() > list2.len() {
        return find_restaurant(list2, list1);
    }

    let map = list1
        .into_iter()
        .enumerate()
        .map(|(idx, string)| (string, idx))
        .collect::<HashMap<String, usize>>();

    let mut min_idx_sum = usize::MAX;
    let mut v: Vec<String> = Vec::new();
    let mut curr: usize;

    for (idx, string) in list2.iter().enumerate() {
        if map.get(string).is_some() {
            curr = idx + *map.get(string).unwrap();
            if curr == min_idx_sum {
                v.push(string.to_string());
            } else if curr < min_idx_sum {
                min_idx_sum = curr;
                v.clear();
                v.push(string.to_string());
            }
        }
    }
    v
}

#[allow(unused)]
pub fn count_max_or_subsets_1(nums: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    for it in nums.iter() {
        max |= it;
    }

    let mut cnt: i32 = 0;
    let mut cur: i32 = 0;

    for i in 1..(1 << nums.len()) {
        cur = 0;
        for (idx, val) in nums.iter().enumerate() {
            if (i >> idx) & 1 == 1 {
                cur |= val;
            }
        }
        if cur == max {
            cnt += 1;
        }
    }
    cnt
}

#[allow(unused)]
pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    fn back_track(nums: &Vec<i32>, start: usize, cur: i32, max: i32, cnt: &mut i32) {
        if cur == max {
            *cnt += 1;
        }
        for i in start..nums.len() {
            back_track(nums, i + 1, cur | nums[i], max, cnt);
        }
    }
    let max = nums.iter().fold(0, |mut max, it| {
        max |= it;
        max
    });
    let mut cnt = 0;
    back_track(&nums, 0, 0, max, &mut cnt);
    cnt
}

#[allow(unused)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(num) => num as i32,
        Err(_) => -1,
    }
}

#[allow(unused)]
pub fn length_of_longest_substring(s: String) -> i32 {
    match s.len() {
        0 | 1 => s.len() as i32,
        _ => {
            let mut cnt = 0;
            let mut start = 0;
            let mut last = vec![-1; 128];
            for (i, ch) in s.chars().enumerate() {
                let idx = ch as usize;
                start = max(start, last[idx] + 1);
                cnt = max(cnt, i - start as usize + 1);
                last[idx] = i as i32;
            }
            cnt as i32
        }
    }
}

#[allow(unused)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    for i in 0..len {
        if nums[i] >= target {
            return i as i32;
        }
    }
    return len as i32;
}

#[allow(unused)]
pub fn contains_duplicate_2(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let hs: HashSet<i32> = HashSet::from_iter(nums.into_iter());
    if hs.len() == len {
        false
    } else {
        true
    }
}

#[allow(unused)]
pub fn contains_duplicate_1(nums: Vec<i32>) -> bool {
    let mut hs = HashSet::new();
    for num in nums.iter() {
        if hs.contains(num) {
            return true;
        }
        hs.insert(num);
    }
    false
}

#[allow(unused)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    nums.iter().any(|&x| !set.insert(x))
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(unused)]
pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    match root {
        Some(root) => {
            let mut node = root.borrow_mut();
            let mut ans = node.val.to_string();

            let ls = tree2str(node.left.take());
            let rs = tree2str(node.right.take());

            if rs.len() > 0 {
                ans.push_str(&format!("({})({})", ls, rs));
            } else if rs.len() == 0 && ls.len() > 0 {
                ans.push_str(&format!("({})", ls));
            }
            ans
        }
        None => "".to_string(),
    }
}

#[allow(unused)]
pub fn winner_of_game(colors: String) -> bool {
    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let v: Vec<char> = colors.chars().collect();
    for i in 1..v.len() - 1 {
        if v[i - 1] == 'A' && v[i] == 'A' && v[i + 1] == 'A' {
            a_cnt += 1;
        } else if v[i - 1] == 'B' && v[i] == 'B' && v[i + 1] == 'B' {
            b_cnt += 1;
        }
    }
    a_cnt > b_cnt
}

#[allow(unused)]
// 没达到题目要求嗷 log(m + n)
pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    nums1.extend(nums2.iter());
    nums1.sort();
    let len = nums1.len();
    match len % 2 {
        0 => (nums1[len / 2 - 1] as f64 + nums1[len / 2] as f64) / 2.0,
        1 => nums1[len / 2] as f64,
        _ => 0.0,
    }
}

// pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {}

#[allow(unused)]
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    vec![0]
}
