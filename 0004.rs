#![allow(dead_code)]
struct Solution;
// method1
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut numbers = [&nums1[..], &nums2[..]].concat();
        numbers.sort_unstable();
        let mid = numbers.len() / 2;
        (numbers[mid - (numbers.len() % 2 == 0) as usize] as f64 + numbers[mid] as f64) / 2.0
    }
}

// method2
// impl Solution {
//     pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//         let (nums1, nums2) = if nums1.len() <= nums2.len() {
//             (nums1, nums2)
//         } else {
//             (nums2, nums1)
//         };

//         let (m, n) = (nums1.len(), nums2.len());
//         let (mut left, mut right) = (0, m);

//         while left <= right {
//             let i = (left + right) / 2;
//             let j = (m + n + 1) / 2 - i;

//             let max_left1 = if i == 0 { std::i32::MIN } else { nums1[i - 1] };
//             let min_right1 = if i == m { std::i32::MAX } else { nums1[i] };
//             let max_left2 = if j == 0 { std::i32::MIN } else { nums2[j - 1] };
//             let min_right2 = if j == n { std::i32::MAX } else { nums2[j] };

//             if max_left1 > min_right2 {
//                 right = i - 1;
//             } else if max_left2 > min_right1 {
//                 left = i + 1;
//             } else {
//                 let max_left = max_left1.max(max_left2);
//                 let min_right = min_right1.min(min_right2);

//                 return if (m + n) % 2 == 1 {
//                     max_left as f64
//                 } else {
//                     (max_left + min_right) as f64 / 2.0
//                 };
//             }
//         }

//         unreachable!();
//     }
// }
