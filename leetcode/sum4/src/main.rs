use std::convert::TryInto;

fn main() {
    let mut nums = Vec::new();
    nums.push(1000000000);
    nums.push(1000000000);
    nums.push(1000000000);
    nums.push(1000000000);
    let target = -294967296;
    println!("{:?}", Solution::four_sum(nums, target))
}
struct Solution {

}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums: Vec<i64> = nums.iter().map(|&n| n.into()).collect();
        nums.sort_unstable();
        println!("{:?} ", nums);
        let my_target: i64 = target.into();
        let res = Solution::reach_target(nums, my_target, 4);
        match res {
            Some(r) => r.iter().map(|v| v.iter().map(|&n| n.try_into().unwrap_or(0)).collect()).collect(),
            None => Vec::new(),
        }
    }

    pub fn reach_target(nums: Vec<i64>, target: i64, remaining: i64) -> Option<Vec<Vec<i64>>>{
        //println!("target {}  || remaining {}", target, remaining);
        
        if remaining <= 0 || nums.len() < remaining.try_into().unwrap_or(nums.len()) {
            //println!("stop, nums too short");
            return None;
        } 

        if remaining == 1 {
            match nums.binary_search(&target) {
                Ok(_) => {
                    //println!("found");
                    let mut res = Vec::new();
                    let mut r = Vec::new();
                    r.push(target);
                    res.push(r);
                    return Some(res);
                },
                Err(_) => {
                    //println!("target not found");
                    return None;
                }
            }
        }

        let mut last = nums[0] - 1;

        let mut result_collection = Vec::new();

        for (pos, &n) in nums.iter().enumerate() {
            if (nums.len() - pos) < (remaining - 1).try_into().unwrap() {
                //println!("stop, not enought remaing");
                break;
            }

            //println!("n: {} || last: {}",n,last);
            if n != last {
                last = n;

                let optional_pr = Solution::reach_target(nums[pos+1 ..].to_vec(), target - n, remaining - 1);

                if let Some(partial_results) = optional_pr {

                    let x: Vec<Vec<i64>> = partial_results.iter().filter(|&pr| pr.len() > 0).map(|v| {
                        let mut x = Vec::new();
                        x.push(n);
                        return [x, v.to_vec()].concat();     
                    }).collect();
                    //println!("Ok {}",n);

                    result_collection = [result_collection, x].concat();
                }
            }
        }
        //println!("stop, not found");

        if result_collection.len() > 0 {
           Some(result_collection)
        } else {
            None
        }
        

    } 
}