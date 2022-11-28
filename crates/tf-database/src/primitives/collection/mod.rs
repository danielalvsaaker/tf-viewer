mod index;
mod relation;
mod tree;

pub use index::Index;
pub use relation::Relation;
pub use tree::Tree;

pub struct Iter<T> {
    iter: std::vec::IntoIter<T>,
    pub total_count: usize,
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

pub fn next_byte_sequence(start: &[u8]) -> Option<Vec<u8>> {
    let mut end = start.to_vec();
    // Modify the last byte by adding one. If it would wrap, we proceed to the
    // next byte.
    while let Some(last_byte) = end.pop() {
        if let Some(next) = last_byte.checked_add(1) {
            end.push(next);
            return Some(end);
        }
    }

    None
}
