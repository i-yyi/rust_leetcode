 // Add missing import statement
mod lc847;
mod lc1043;
mod lc1337;
mod lc287;
mod lc1658;
mod lc3033;
mod lc3034;


struct Solution{}
fn main() {
    // Solution::shortest_path_length(vec![vec![1,2,3],vec![0],vec![0],vec![0]]);
    // let res = Solution::min_operations(vec![2, 3, 1, 1, 1], 5);
    // println!("res:{}", res);
    let res = Solution::count_matching_subarrays(vec![5, 4, 5, 4, 4, 3], vec![-1, 1, -1, 0, -1]);
    println!("res:{}", res);
}
