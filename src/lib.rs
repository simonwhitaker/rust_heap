use std::cmp::PartialOrd;

pub struct Heap<T> {
    storage: Vec<T>,
}

impl <T:PartialOrd> Heap<T> {
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

    pub fn remove_first(&mut self) -> Option<T> {
        if self.storage.len() > 0 {
            let result = self.storage.swap_remove(0);
            self.sift_down(0);
            return Some(result);
        }
        return None
    }

    fn sift_up(&mut self, from_index: usize) {
        let mut idx = from_index;
        while idx > 0 {
            let pidx = (idx - 1) / 2;
            if self.storage[idx] < self.storage[pidx] {
                self.storage.swap(idx, pidx);
            }
            idx = pidx;
        }
    }

    fn sift_down(&mut self, from_index: usize) {
        let idx = from_index;
        for cidx in 2 * idx + 1 .. 2 * idx + 2 {
            if cidx < self.storage.len() {
                if self.storage[idx] > self.storage[cidx] {
                    self.storage.swap(idx, cidx);
                    self.sift_down(cidx);
                }
            }
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

    #[test]
    fn test_remove_first() {
        let mut heap: Heap<i32> = Heap::new();
        heap.add(3);
        heap.add(2);
        heap.add(1);
        assert_eq!(heap.remove_first(), Some(1i32));
        assert_eq!(heap.remove_first(), Some(2i32));
        assert_eq!(heap.remove_first(), Some(3i32));
        assert_eq!(heap.remove_first(), None);
    }
}
