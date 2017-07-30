pub struct Heap<T> {
    storage: Vec<T>,
}

impl <T> Heap<T> {
    pub fn new() -> Self {
        let mut _storage = Vec::new();
        return Heap { storage: _storage };
    }

    fn add(&mut self, element: T) {
        self.storage.push(element);
    }

    fn top(&self) -> Option<&T> {
        return self.storage.get(0);
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
        let top = heap.top().expect("Ugh");

        assert_eq!(*top, val);
    }
}
