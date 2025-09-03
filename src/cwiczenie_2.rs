use std::hash::Hash;
use std::collections::HashSet;

pub fn unique<T: Ord + Clone + Hash>(elements: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for element in elements {
        if seen.insert(element.clone()) {
            deduped.push(element);
        }
    }
    deduped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_duplicated() {
        let elements = vec![1, 7, 3, 5];
        assert_eq!(unique(elements), vec![1,7,3,5])
    }

    #[test]
    fn with_duplicated() {
        let elements = vec![1, 6, 2, 2, 2, 3, 5];
        assert_eq!(unique(elements), vec![1,6,2,3,5]);
    }

    #[test]
    fn emtpy() {
        let elements: Vec<i32> = vec![];
        assert_eq!(unique(elements), vec![]);
    }
}
