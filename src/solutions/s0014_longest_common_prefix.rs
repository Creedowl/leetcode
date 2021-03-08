use super::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 { return "".to_string(); }
        let mut result = String::new();
        let mut current: char;
        let mut index = 0;
        let first = strs.first().unwrap();
        for ch in first.chars() {
            current = ch;
            for str in strs.iter().skip(1) {
                if index >= str.len() || current != *str.as_bytes().get(index).unwrap() as char {
                    return result;
                }
            }
            result.push(ch);
            index += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn s14_test() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_string());
        assert_eq!(Solution::longest_common_prefix(
            vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string()
        );
        assert_eq!(Solution::longest_common_prefix(
            vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string()
        );
    }
}