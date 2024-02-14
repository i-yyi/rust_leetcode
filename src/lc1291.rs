impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res: Vec<_> = (1..=9)
        .flat_map(|i| (i+1..=9).map(move |l| (i, l)))
        .filter_map(|(i, l)| {
            let now = (i..=l).fold(0, |now, cur| now * 10 + cur);
            if now >= low && now <= high { Some(now) } else { None }
            })
        .collect();

        res.sort();
        res
    }
}