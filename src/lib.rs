fn bubble_sort(values: &mut [u64]) {
    for already_sorted in 1..values.len() {
        for i in 0..(values.len() - already_sorted) {
            if values[i] > values[i + 1] {
                (values[i], values[i + 1]) = (values[i + 1], values[i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut values = vec![
            5, 1, 2, 8, 9, 7, 15, 3, 59, 192, 1, 5, 1, 298, 18, 7, 8, 9, 159,
        ];

        bubble_sort(&mut values);
        assert_eq!(
            values,
            vec![1, 1, 1, 2, 3, 5, 5, 7, 7, 8, 8, 9, 9, 15, 18, 59, 159, 192, 298]
        )
    }
}
