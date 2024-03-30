use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div};

pub type FieldSize = u16;


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FieldElement<'a> {
    element: FieldSize,
    finite_field: &'a FiniteField,
}


impl<'a> Add for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            element: self.element + rhs.element,
            finite_field: self.finite_field
        }
    }
}

impl<'a> Sub for FieldElement<'a> {
    type Output = FieldElement<'a>;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.element > self.element {
            Self {
                element: self.finite_field.prime - self.element - rhs.element,
                finite_field: self.finite_field,
            }
        } else {
            Self {
                element: self.element - rhs.element,
                finite_field: self.finite_field
            }
        }
    }
}

impl<'a> Mul for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            element: (self.element * rhs.element) % self.finite_field.prime,
            finite_field: self.finite_field,
        }
    }
}

impl<'a> Div for FieldElement<'a> {
    type Output = FieldElement<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.element == 0 || rhs.element == 0 {
            Self {
                element: 0,
                finite_field: self.finite_field,
            }
        } else {
            Self {
                element: self.element * rhs.inverse().element % self.finite_field.prime,
                finite_field: self.finite_field,
            }
        }
    }
}

impl<'a> FieldElement<'a> {
    pub fn inverse(&self) -> Self {
        todo!()
    }
}


#[derive(PartialEq, Debug)]
pub struct FiniteField {
    pub prime: FieldSize,
}

impl FiniteField {
    pub fn new(prime: FieldSize) -> Self {
        Self {
            prime
        }
    }

    pub fn element(&self, value: FieldSize) -> FieldElement {
        FieldElement {
            element: value,
            finite_field: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_field::{FiniteField};

    #[test]
    fn test_finite_field() {
        let finite_field = FiniteField::new(97u16);
        let field_element1 = finite_field.element(6);
        let field_element2 = finite_field.element(3);

        assert_eq!(field_element1 + field_element2, finite_field.element(9));
        assert_eq!(field_element1 - field_element2, finite_field.element(3));
        assert_eq!(field_element1 * field_element2, finite_field.element(18));
    }
}
