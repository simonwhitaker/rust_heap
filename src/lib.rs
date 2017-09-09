use std::cmp::PartialOrd;

pub struct Heap<T> {
    storage: Vec<T>,
}

impl <T:PartialOrd + Copy> Heap<T> {
    pub fn new() -> Self {
        let mut _storage = Vec::new();
        return Heap { storage: _storage };
    }

    pub fn add(&mut self, element: T) {
        self.storage.push(element);
        let max_index = self.storage.len() - 1;
        self.sift_up(max_index);
    }

    pub fn top(&self) -> Option<&T> {
        return self.storage.get(0);
    }

    fn sift_up(&mut self, from_index: usize) {
        let mut idx = from_index;
        let mut pidx: usize;
        while idx > 0 {
            pidx = (idx - 1) / 2;
            let val = self.storage[idx];
            let pval = self.storage[pidx];
            if !(pval < val) {
                self.storage[pidx] = val;
                self.storage[idx] = pval;
            }
            idx = pidx;
        }
    }
}

#[cfg(test)]
mod tests {
    use Heap;

    #[test]
    fn test_add_and_get() {
        let mut heap: Heap<i32> = Heap::new();
        let val: i32 = 42;

        heap.add(val.clone());
        let top = heap.top();

        assert_eq!(top, Some(&val));
    }

    #[test]
    fn test_ordering_maintained() {
        let mut heap: Heap<i32> = Heap::new();
        heap.add(2);
        heap.add(1);
        let top = heap.top().expect("top() returned none");
        assert_eq!(*top, 1i32);
    }
}
