// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    const H:[char;16] = [
        '0', '1', '2', '3', '4', '5',
        '6', '7', '8', '9', 'a', 'b',
        'c', 'd', 'e', 'f'
    ];
    pub fn to_hex(num: i32) -> String {
        let mut num = num as u32;
        if num == 0 {
            return "0".to_owned();
        }
        let mut ret = String::new();
        while num > 0 {
            ret.push(Solution::H[(num % 16) as usize]);
            num /= 16;
        }
        ret.chars().rev().collect()
    }
}

