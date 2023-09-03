use rand::Rng;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, PartialOrd, Deserialize, Serialize)]
pub struct Code(u32);

impl From<Code> for u32 {
    fn from(id: Code) -> Self {
        id.0
    }
}

impl Code {
    pub fn new(digits: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut code = 0;
        for _ in 0..digits {
            code = code * 10 + rng.gen_range(0..10);
        }
        Self(code)
    }
}

#[cfg(test)]
mod tests_code {
    use super::*;

    #[test]
    fn test_4_digits() {
        let code = Code::new(4);
        assert!(code > Code(999) && code < Code(10000));
    }
    
    #[test]
    fn test_6_digits() {
        let code = Code::new(6);
        assert!(code > Code(99999) && code < Code(1000000));
    }
    
    #[test]
    fn test_8_digits() {
        let code = Code::new(8);
        assert!(code > Code(9999999) && code < Code(100000000));
    }
}
