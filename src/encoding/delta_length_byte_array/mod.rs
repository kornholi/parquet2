mod decoder;
mod encoder;

pub use decoder::Decoder;
pub use encoder::encode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let data = vec!["aa", "bbb", "a", "aa", "b"];

        let mut buffer = vec![];
        encode(data.into_iter().map(|x| x.as_bytes()), &mut buffer);

        let mut iter = Decoder::new(&buffer);

        let result = iter.by_ref().collect::<Vec<_>>();
        assert_eq!(result, vec![2, 3, 1, 2, 1]);

        let result = iter.into_values();
        assert_eq!(result, b"aabbbaaab".as_ref());
    }

    #[test]
    fn many_numbers() {
        let mut data = vec![];
        for i in 0..136 {
            data.push(format!("a{}", i))
        }
        let expected_values = data.join("");
        let expected_lengths = data.iter().map(|x| x.len() as i32).collect::<Vec<_>>();

        let mut buffer = vec![];
        encode(data.into_iter(), &mut buffer);

        let mut iter = Decoder::new(&buffer);

        let result = iter.by_ref().collect::<Vec<_>>();
        assert_eq!(result, expected_lengths);

        let result = iter.into_values();
        assert_eq!(result, expected_values.as_str().as_bytes());
    }
}
