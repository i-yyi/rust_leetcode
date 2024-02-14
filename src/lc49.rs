use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_table = HashMap::new();
        for i in &strs {
            let sorted_str = i.chars().collect::<Vec<_>>().sort();
            println!("{:?}", sorted_str);
            hash_table.entry(sorted_str).or_insert(Vec::new()).push(i.to_string());
        }
        hash_table.values().cloned().collect()
    }
}