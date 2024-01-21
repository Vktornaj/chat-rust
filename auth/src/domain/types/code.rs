use rand::Rng;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, PartialOrd, Serialize)]
pub struct Code(String);

impl Code {
    pub fn new(digits: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut code = "".to_owned();
        for _ in 0..digits {
            code.push_str(rng.gen_range(0..10).to_string().as_str());
        }
        Self(code)
    }
    
    pub fn new_0s(digits: u8) -> Self {
        let mut code = "".to_owned();
        for _ in 0..digits {
            code.push_str("0");
        }
        Self(code)
    }
}

impl From<String> for Code {
    fn from(s: String) -> Code {
        Code(s)
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Code, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Code(s))
    }
}

impl From<Code> for String {
    fn from(id: Code) -> Self {
        id.0
    }
}

impl PartialEq<String> for Code {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests_code {
    use super::*;

    #[test]
    fn test_4_digits() {
        let code = Code::new(4);
        assert!(code > Code("999".to_owned()) && code < Code("10000".to_owned()));
    }
    
    #[test]
    fn test_6_digits() {
        let code = Code::new(6);
        assert!(code > Code("99999".to_owned()) && code < Code("1000000".to_owned()));
    }
    
    #[test]
    fn test_8_digits() {
        let code = Code::new(8);
        assert!(code > Code("9999999".to_owned()) && code < Code("100000000".to_owned()));
    }
}
