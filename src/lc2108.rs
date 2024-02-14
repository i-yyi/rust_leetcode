impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let check_if_palindrome = |word: &String| {
            let word: Vec<_> = word.chars().collect();
            word.iter().zip(word.iter().rev()).all(|(a, b)| a == b)
        };        
        words.iter()
        .find(|&word| Self::check_if_palindrome(word))
        .unwrap_or(&"".to_string())
        .clone()
    }
}