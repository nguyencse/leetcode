use std::collections::HashMap;

fn main() {
    let problem = Problem {
        nums: vec![2, 7, 11, 15],
        target: 9,
    };
    // let problem = Problem { nums: vec![3,2,4], target: 6 };
    // let problem = Problem { nums: vec![3,3], target: 6 };
    let result = problem.two_sum();

    println!("{:?}", result);
}

struct Problem {
    nums: Vec<i32>,
    target: i32,
}

trait Solution {
    fn two_sum(self) -> Vec<i32>;
}

impl Solution for Problem {
    fn two_sum(self) -> Vec<i32> {
        let mut prev_map: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in self.nums.iter().enumerate() {
            let remain = self.target - num;

            if prev_map.contains_key(&remain) {
                return vec![i as i32, prev_map[&remain] as i32];
            }
            prev_map.insert(num, i);
        }

        return vec![];
    }
}
