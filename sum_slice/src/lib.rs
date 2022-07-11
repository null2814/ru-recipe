pub fn sum(inputs: &[u32]) -> Option<u32> {
    let mut result = 0u32;
    let mut is_overflow = false;
    for &input in inputs {
        let remain = u32::MAX - result;
        match remain >= input {
            true => result += input,
            false => is_overflow = true,
        }
    }
    if is_overflow {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = vec![1u32, 2u32, 3u32];
        assert_eq!(sum(&result[..]), Some(6));

        let result = vec![1u32, u32::MAX, 3u32];
        assert_eq!(sum(&result[..]), None);
    }
}
