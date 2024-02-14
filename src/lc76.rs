impl Solution {
    fn count_alphabet(s: &String) -> Vec<i32>{
        s.chars()
        .map(|ch| ch as usize - 'A' as usize)
        .fold(vec![0; 52], |mut count, i| {
            count[i] += 1;
            count
        })
    }
    fn check(len: usize, s: &String, count_t: &Vec<i32>) -> (bool, usize, usize) {

        let (mut res, mut res_l, mut res_r) = (false, 0, 0);
        let is_greater_or_equal = |s: &Vec<i32>, t: &Vec<i32>| -> bool {
            s.iter().zip(t.iter()).all(|(a, b)| a >= b)
        };
        
        let s: Vec<char> = s.chars().collect();
        let mut count_s = Self::count_alphabet(&s[..len].iter().collect());
        
        if is_greater_or_equal(&count_s, count_t) {
            return (true, 0, len-1);
        }
        for i in len..s.len() { // enum the terminal one 
            count_s[s[i] as usize - 'A' as usize] += 1;
            count_s[s[i-len] as usize - 'A' as usize] -= 1;
            if is_greater_or_equal(&count_s, count_t) {
                res = true;
                res_l = i-len+1;
                res_r = i;
            }
        }
        (res, res_l, res_r)
                       
    }
    pub fn min_window(s: String, t: String) -> String {
        let count_t = Self::count_alphabet(&t);
        let (mut l, mut r, mut ans_l, mut ans_r) = (1, s.len(), 0, 0);

        while l <= r {
            let mid = (l + r) >> 1;
            let (res, res_l, res_r) = Self::check(mid, &s, &count_t);
            println!("check {} : {} {} {}", mid, res, res_l, res_r);
            if res {
                (ans_l, ans_r) = (res_l, res_r);
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        s[ans_l..=ans_r].to_string()
    }
}