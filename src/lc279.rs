impl Solution {
    fn dfs(dep: i32, now: i32, target: i32, numbers: &Vec<i32>) -> i32 {
        if now == target {
            return dep;
        }

        for i in numbers {
            if now + i <= target {
                let res = dfs(dep+1, now+i, target, numbers);
                if res != 0 {
                    return res;
                }
            }
        }

        return 0;
    }
    pub fn num_squares(n: i32) -> i32 {
        let square_numbers = (1..=100).map(|x| x * x).collect().rev();
        dfs(0, 0, n, square_numbers)
    }   
}