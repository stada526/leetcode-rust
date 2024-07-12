use crate::common::heap::MaxHeap;

struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = MaxHeap::new(stones);
        while max_heap.len() > 1 {
            let y = max_heap.pop().unwrap();
            let x = max_heap.pop().unwrap();
            let diff = y - x;
            if diff != 0 {
                max_heap.push(diff)
            }
        }

        return if max_heap.len() == 0 {
            0
        } else {
            max_heap.pop().unwrap()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let res = Solution::last_stone_weight(stones);
        assert_eq!(res, 1)
    }
}
