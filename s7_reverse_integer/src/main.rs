fn main() {
    // let result = Solution::reverse(-123);
    // let result = Solution::reverse(120);
    let result = Solution::reverse(1534236469);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x < i32::MIN || x > (i32::MAX - 1) || x == 0 {
            return 0;
        }

        let is_negative = x < 0;
        let n = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);

        match n {
            0 => 0,
            n if is_negative => -n,
            _ => n,
        }
    }
}
