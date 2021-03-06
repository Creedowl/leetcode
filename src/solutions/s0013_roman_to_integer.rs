use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let numbers: Vec<i32> = s.chars().map(|ch| {
            match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("invalid")
            }
        }).collect();
        let mut result = 0;
        for i in 0..numbers.len() - 1 {
            let num = numbers[i];
            result += if num < numbers[i + 1] { -num } else { num }
        }
        result + numbers.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    #[ignore]
    fn s13_test() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}