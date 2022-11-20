fn main() {
    // let result = Solution::my_atoi("   -42".to_string());
    // let result = Solution::my_atoi("   4193 with words".to_string());
    // let result = Solution::my_atoi("words and 987".to_string());
    // let result = Solution::my_atoi("+-12".to_string());
    // let result = Solution::my_atoi("-+12".to_string());
    // let result = Solution::my_atoi("+1".to_string());
    // let result = Solution::my_atoi("21474836460".to_string());
    // let result = Solution::my_atoi("2147483648".to_string());
    let result = Solution::my_atoi("9223372036854775808".to_string());
    println!("{:?}", result);
}

struct Solution;

fn format_number(num_64: i64, is_negative: bool) -> (i32, bool) {
    if is_negative {
        if -num_64 < (i32::MIN as i64) {
            (i32::MIN, true)
        } else {
            (-(num_64 as i32), false)
        }
    } else {
        if num_64 > (i32::MAX as i64) {
            (i32::MAX, true)
        } else {
            ((num_64 as i32), false)
        }
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_negative = false;
        let mut has_num = false;
        let mut num: i64 = 0;

        for (i, c) in s.trim().chars().enumerate() {
            if c == '-' && i == 0 {
                is_negative = true;
            } else {
                if c == '+' && i == 0 {
                    continue;
                }
                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap() as i64;
                    let (num_32, is_overflow) = format_number(num, false);
                    has_num = true;

                    if is_overflow {
                        break;
                    } else {
                        num = num_32 as i64;
                    }

                } else if has_num {
                    break;
                } else {
                    return 0;
                }
            }
        }
        format_number(num, is_negative).0
    }
}
