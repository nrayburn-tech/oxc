use crate::{StringCharAt, string_char_at::StringCharAtResult};

pub trait StringCharCodeAt {
    /// `String.prototype.charCodeAt ( pos )`
    /// <https://tc39.es/ecma262/#sec-string.prototype.charcodeat>
    fn char_code_at(&self, index: Option<f64>) -> Option<u32>;
}

impl StringCharCodeAt for &str {
    fn char_code_at(&self, index: Option<f64>) -> Option<u32> {
        match self.char_at(index) {
            StringCharAtResult::Value(c) => Some(c as u32),
            StringCharAtResult::InvalidChar(v) => Some(u32::from(v)),
            StringCharAtResult::OutOfRange => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::StringCharCodeAt;

    #[test]
    fn test_evaluate_char_code_at() {
        let s = "abcde";
        assert_eq!(s.char_code_at(Some(0.0)), Some(97));
        assert_eq!(s.char_code_at(Some(1.0)), Some(98));
        assert_eq!(s.char_code_at(Some(2.0)), Some(99));
        assert_eq!(s.char_code_at(Some(3.0)), Some(100));
        assert_eq!(s.char_code_at(Some(4.0)), Some(101));
        assert_eq!(s.char_code_at(Some(5.0)), None);
        assert_eq!(s.char_code_at(Some(-1.0)), None);
        assert_eq!(s.char_code_at(None), Some(97));
        assert_eq!(s.char_code_at(Some(0.0)), Some(97));
        assert_eq!(s.char_code_at(Some(f64::NAN)), Some(97));
        assert_eq!(s.char_code_at(Some(f64::INFINITY)), None);
    }
}
