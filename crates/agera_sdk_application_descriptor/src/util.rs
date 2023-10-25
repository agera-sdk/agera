/// Provides additional methods for the standard `Vec<T>` type.
pub(crate) trait VectorExtensions<T> {
    /// Finds the index of a value.
    fn index_of(&self, value: &T) -> Option<usize> where T: PartialEq;

    /// Finds the index of a value starting from `start_index`.
    fn index_of_from(&self, value: &T, start_index: usize) -> Option<usize> where T: PartialEq;

    /// Removes an element that meets the criteria `element == value`.
    fn remove_equals(&mut self, value: &T) -> bool  where T: PartialEq;
}

impl<T> VectorExtensions<T> for Vec<T> {
    fn index_of(&self, value: &T) -> Option<usize> where T: PartialEq {
        for i in 0..self.len() {
            if self[i] == *value {
                return Some(i);
            }
        }
        None
    }

    fn index_of_from(&self, value: &T, start_index: usize) -> Option<usize> where T: PartialEq {
        if start_index >= self.len() {
            return None;
        }
        for i in start_index..self.len() {
            if self[i] == *value {
                return Some(i);
            }
        }
        None
    }

    fn remove_equals(&mut self, value: &T) -> bool  where T: PartialEq {
        let i = self.index_of(value);
        if let Some(i) = i {
            self.remove(i);
            true
        } else {
            false
        }
    }
}