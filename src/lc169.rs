impl Solution {
    fn check(len: usize, s: &String, t: &String) -> (bool, usize, usize) {

        println!(" Check {} {} {}", len, s, t);
        let is_greater_or_equal = |s: Vec<i32>, t: Vec<i32>| -> bool {
            s.into_iter().zip(t.into_iter()).all(|(a, b)| a >= b)
        };
        println!(" {:?} ", t.chars());
        let count_t = t.chars()
                       .map(|ch| ch as usize - 'A' as usize)
                       .fold(vec![0; 52], |mut count, i| {
                            count[i] += 1;
                            count
                        });
                        println!("--");
        let mut count_s = s[..len].chars()
                              .map(|ch| ch as usize - 'A' as usize)
                              .fold(vec![0; 52], |mut count, i| {
                                count[i] += 1;
                                count
                              });
        println!(" init counts {:?} \n {:?}", count_t, count_s);
        let mut res = false;
        let mut lres = 0;
        let mut rres = 0;
        let s: Vec<char> = s.chars().collect();
        for i in len..s.len() {
            count_s[s[i] as usize - 'A' as usize] += 1;
            count_s[s[i-len] as usize - 'A' as usize] -= 1;
            if is_greater_or_equal(count_s.clone(), count_t.clone()) {
                res = true;
                lres = i-len+1;
                rres = i;
            }
        }
        (res, lres, rres)
                       
    }
    pub fn min_window(s: String, t: String) -> String {
        let (lens, lent) = (s.len(), t.len());
        let (mut l, mut r) = (0, lens);
        let mut ansl = 0;
        let mut ansr = 0;
        while l < r {
            let mid = (l + r) >> 1;
            let (res, lres, rres) = Self::check(mid, &s, &t);
            if (res) {
                r = mid - 1;
                ansl = lres;
                ansr = rres;
            } else {
                l = mid + 1;
            }
        }
        s
    }
}