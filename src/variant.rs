use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Variant {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    ArrayRef(Rc<RefCell<Vec<Variant>>>),
    TableRef(Rc<RefCell<HashMap<String, Variant>>>),
}

impl Variant {
    pub fn as_bool(&self) -> bool {
        match self {
            Variant::Boolean(b) => *b,
            Variant::Integer(i) => *i != 0,
            Variant::Float(f) => *f != 0.0,
            Variant::String(s) => !s.is_empty(),
            _ => false
        }
    }
    
    pub fn pow(&self, rhs: &Variant) -> Variant {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs.pow(*rhs as u32)),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs.powf(*rhs)),
            _ => panic!("Invalid operands for exponentiation")
        }
    }
}

impl Into<i64> for Variant {
    fn into(self) -> i64 {
        match self {
            Variant::Integer(i) => i,
            v => panic!("Cannot convert from {:?} to i64", v)
        }
    }
}

impl Into<f64> for Variant {
    fn into(self) -> f64 {
        match self {
            Variant::Float(f) => f,
            v => panic!("Cannot convert from {:?} to f64", v)
        }
    }
}

impl Into<String> for Variant {
    fn into(self) -> String {
        match self {
            Variant::String(s) => s,
            _ => panic!("Cannot convert to String")
        }
    }
}

impl Into<bool> for Variant {
    fn into(self) -> bool {
        match self {
            Variant::Boolean(b) => b,
            Variant::Integer(i) => i != 0,
            Variant::Float(f) => f != 0.0,
            Variant::String(s) => !s.is_empty(),
            _ => panic!("Cannot convert to bool")
        }
    }
}


impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => lhs == rhs,
            (Variant::Float(lhs), Variant::Float(rhs)) => lhs == rhs,
            (Variant::String(lhs), Variant::String(rhs)) => lhs == rhs,
            (Variant::Boolean(lhs), Variant::Boolean(rhs)) => lhs == rhs,
            (Variant::Identifier(lhs), Variant::Identifier(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

// Add Operator trait to Variant
impl Add for Variant {
    type Output = Variant;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs + rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs + rhs),
            (Variant::String(lhs), Variant::String(rhs)) => Variant::String(lhs + &rhs),
            (Variant::Boolean(lhs), Variant::Boolean(rhs)) => Variant::Boolean(lhs || rhs),
            _ => panic!("Invalid operands for addition")
        }
    }
}

impl Sub for Variant {
    type Output = Variant;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs - rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs - rhs),
            _ => panic!("Invalid operands for subtraction")
        }
    }
}

impl Div for Variant {
    type Output = Variant;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs / rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs / rhs),
            _ => panic!("Invalid operands for division")
        }
    }
}

impl Mul for Variant {
    type Output = Variant;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs * rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs * rhs),
            _ => panic!("Invalid operands for multiplication")
        }
    }
}

impl Rem for Variant {
    type Output = Variant;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => Variant::Integer(lhs % rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => Variant::Float(lhs % rhs),
            _ => panic!("Invalid operands for modulus")
        }
    }
}

impl Neg for Variant {
    type Output = Variant;

    fn neg(self) -> Self::Output {
        match self {
            Variant::Integer(i) => Variant::Integer(-i),
            Variant::Float(f) => Variant::Float(-f),
            Variant::Boolean(b) => Variant::Boolean(!b),
            _ => panic!("Invalid operand for negation")
        }
    }
}

impl Not for Variant {
    type Output = Variant;

    fn not(self) -> Self::Output {
        match self {
            Variant::Boolean(b) => Variant::Boolean(!b),
            Variant::Integer(i) => Variant::Boolean(i == 0),
            Variant::Float(f) => Variant::Boolean(f == 0.0),
            Variant::String(s) => Variant::Boolean(s.is_empty()),
            _ => panic!("Invalid operand for not operation")
        }
    }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Variant::Integer(lhs), Variant::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Variant::Float(lhs), Variant::Float(rhs)) => lhs.partial_cmp(rhs),
            _ => None
        }
    }
}