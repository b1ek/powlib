#[derive(Clone, Copy)]
pub struct Num {
    number: u128
}

impl Num {
    pub fn new(num: u128) -> Num {
        Num { number: num }
    }

    pub fn bytes(self: &Num) -> [u8; 16] {
        self.number.to_be_bytes()
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Num {
        Num { number: u128::from_be_bytes(bytes) }
    }
}

impl std::ops::Mul<u128> for Num {
    type Output = u128;
    fn mul(self, rhs: u128) -> Self::Output {
        self.number * rhs
    }
}

impl From<u128> for Num {
    fn from(number: u128) -> Self {
        Num { number }
    }
}

impl From<Num> for u128 {
    fn from(value: Num) -> Self {
        value.number
    }
}