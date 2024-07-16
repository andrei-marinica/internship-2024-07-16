use num_bigint::BigUint;

/// This is docs.
pub fn factorial1(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

pub fn factorial2(mut n: BigUint) -> BigUint {
    let mut result = BigUint::from(1u32);
    let one = BigUint::from(1u32) ;
    while n > one {
        result *= &n;
        n -= &one;
    }

    result
}


#[derive(Clone, PartialEq, Eq)]
pub struct FactorialComputation {
    input: BigUint,
    output: Option<BigUint>,
}

impl FactorialComputation {
    pub fn new(input: BigUint) -> Self {
        FactorialComputation {
            input,
            output: None,
        }
    }

    pub fn compute(self) -> BigUint {
        factorial2(self.input)
    }

    pub fn compute_ref(&self) -> BigUint {
        factorial2(self.input.clone())
    }

    pub fn compute_mut(&mut self) {
        let f = self.compute_ref();
        self.output = Some(f)
    }

    pub fn inc_input(&mut self) {
        self.input += 1u32;
    }

    pub fn pretty(&self) {
        if let Some(o) = &self.output {
            println!("{}! = {o}", self.input);
        } else {
            println!("{}! = ???", self.input);
        }
    }
}

impl Default for FactorialComputation {
    fn default() -> Self {
        Self {
            input: BigUint::from(5u32),
            output: None,
        }
    }
}

impl From<u32> for FactorialComputation {
    fn from(value: u32) -> Self {
        Self {
            input: BigUint::from(value),
            output: None,
        }
    }
}

impl From<u64> for FactorialComputation {
    fn from(value: u64) -> Self {
        Self {
            input: BigUint::from(value),
            ..Default::default()
        }
    }
}
