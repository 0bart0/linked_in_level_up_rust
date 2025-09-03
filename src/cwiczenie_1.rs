pub fn median(numbers: &[f32]) -> Option<f32> {
    if numbers.is_empty() {
        return None;
    }
    let mut nums = numbers.to_owned();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let nums_size = nums.len();
    if nums_size % 2 == 0 {
        let first_index: usize = nums_size / 2 - 1;
        let second_index: usize = nums_size / 2;
        let res: f32 = (nums[first_index] + nums[second_index]) / 2.0;
        Some(res)
    } else {
        let index: usize = nums_size / 2;
        let res: f32 = nums[index];
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: Vec<f32> = vec![];
        assert_eq!(median(&list), None);
    }

    #[test]
    fn test_odd_size() {
        let list: Vec<f32> = vec![1.0, 4.0, 5.0];
        assert_eq!(median(&list), Some(4.0));
    }

    #[test]
    fn test_even_size() {
        let list: Vec<f32> = vec![1.5, 3.0, 5.0, 8.8];
        assert_eq!(median(&list), Some(4.0));
    }
}
