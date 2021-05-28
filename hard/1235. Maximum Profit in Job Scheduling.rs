impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {  
        let n: usize = start_time.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|x, y| {
            end_time[*x].cmp(&end_time[*y]) 
        });
        
        let mut sliding_max: Vec<i32> = vec![0; n];
        sliding_max[0] = profit[idx[0]];
        for i in 1..n {
            sliding_max[i] = profit[idx[i]];
            
            let mut start: i32 = 0;
            let mut end: i32 = i as i32 - 1;
            let mut ans: i32 = -1;
            
            while start <= end {
                let mid = (start + end) >> 1;
                if end_time[idx[mid as usize]] <= start_time[idx[i]] {
                    ans = mid;
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            
            if ans != -1 {
                sliding_max[i] += sliding_max[ans as usize];           
            }
            sliding_max[i] = std::cmp::max(sliding_max[i], sliding_max[i - 1]);
        }
        
        sliding_max[n - 1]
    }
}
