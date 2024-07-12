pub struct MaxHeap {
    tree: Vec<i32>,
}

impl MaxHeap {
    pub fn new(array: Vec<i32>) -> Self {
        let mut heap = Self { tree: array };
        for i in (0..(heap.tree.len() / 2)).rev() {
            heap.heapify(i)
        }

        return heap;
    }

    pub fn push(&mut self, value: i32) {
        self.tree.push(value);

        let mut target_index = self.tree.len() - 1;
        while target_index > 0 {
            let parent_index = (target_index - 1) / 2;
            if self.tree[parent_index] >= self.tree[target_index] {
                break;
            }
            (self.tree[parent_index], self.tree[target_index]) =
                (self.tree[target_index], self.tree[parent_index]);
            target_index = parent_index;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.tree.is_empty() {
            return None;
        }
        let ret = self.tree[0];

        self.tree[0] = *self.tree.last().unwrap();
        self.tree.pop();

        self.heapify(0);

        return Some(ret);
    }

    fn heapify(&mut self, index: usize) {
        let left_index = 2 * index + 1;
        let right_index = 2 * index + 2;

        let mut max_index = index;

        if left_index < self.tree.len() && self.tree[max_index] < self.tree[left_index] {
            max_index = left_index
        }
        if right_index < self.tree.len() && self.tree[max_index] < self.tree[right_index] {
            max_index = right_index
        }
        if index != max_index {
            (self.tree[index], self.tree[max_index]) = (self.tree[max_index], self.tree[index]);
            self.heapify(max_index)
        }
    }

    pub fn len(&self) -> usize {
        return self.tree.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_heap() {
        let max_heap = MaxHeap::new(vec![3, 1, 6, 5, 2, 4]);
        assert_eq!(max_heap.tree, vec![6, 5, 4, 1, 2, 3]);
    }

    #[test]
    fn test_push() {
        let mut max_heap = MaxHeap::new(vec![3, 1, 6, 5, 2, 4]);
        max_heap.push(10);
        assert_eq!(max_heap.tree, vec![10, 5, 6, 1, 2, 3, 4]);
    }

    #[test]
    fn test_pop() {
        let mut max_heap = MaxHeap::new(vec![3, 1, 6, 5, 2, 4]);
        assert_eq!(max_heap.pop(), Some(6));
        assert_eq!(max_heap.tree, vec![5, 3, 4, 1, 2]);

        assert_eq!(max_heap.pop(), Some(5));
        assert_eq!(max_heap.tree, vec![4, 3, 2, 1]);

        assert_eq!(max_heap.pop(), Some(4));
        assert_eq!(max_heap.tree, vec![3, 1, 2]);

        assert_eq!(max_heap.pop(), Some(3));
        assert_eq!(max_heap.tree, vec![2, 1]);

        assert_eq!(max_heap.pop(), Some(2));
        assert_eq!(max_heap.tree, vec![1]);

        assert_eq!(max_heap.pop(), Some(1));
        assert_eq!(max_heap.tree, vec![]);

        assert_eq!(max_heap.pop(), None);
    }
}
