use algebra::finite_field::{FieldElement, FieldSize, FiniteField};
use ndarray::{arr1, s, Array1, Array2, Axis};

pub trait Hasher<'a> {
    fn hash(&self, value: FieldElement<'a>) -> FieldElement<'a>;
}

struct RescueHash<'a> {
    alpha: FieldElement<'a>,
    alpha_inv: FieldElement<'a>,
    finite_field: &'a FiniteField,
    rate: usize,
    capacity: usize,
    mds_matrix: Array2<FieldElement<'a>>,
    constants: Array1<FieldElement<'a>>,
}

impl<'a> Hasher<'a> for RescueHash<'a> {
    fn hash(&self, value: FieldElement<'a>) -> FieldElement<'a> {
        let state_len: usize = self.rate + self.capacity;
        let t: Array1<FieldElement<'a>> = arr1(&[self.finite_field.zero()])
            * arr1(&[self.finite_field.element(state_len as FieldSize)]);

        let mut state: Array1<FieldElement<'a>> = arr1(&[value]);
        state
            .append(Axis(0), t.slice(s![..]))
            .expect("Can't append");

        state.map(|x| x.pow(&self.alpha)); // S-box function

        // round 1
        let mut temp = Array1::<FieldElement<'a>>::from_elem(state_len, self.finite_field.zero());

        for i in 0..state_len {
            for j in 0..state_len {
                temp[i] = temp[i] + &self.mds_matrix[[i, j]] * &state[j];
            }
        }

        for (i, el) in &mut state.iter_mut().enumerate() {
            *el = temp[i] + self.constants[2 * self.rate * state_len + i].abs();
        }

        state.map(|x| x.pow(&self.alpha_inv)); // S-box function
                                               // round 2
        let mut temp = Array1::<FieldElement>::from_elem(state_len, self.finite_field.zero());

        for i in 0..state_len {
            for j in 0..state_len {
                temp[i] = temp[i] + &self.mds_matrix[[i, j]] * &state[j];
            }
        }

        for (i, el) in &mut state.iter_mut().enumerate() {
            *el = temp[i] + self.constants[2 * self.rate * state_len + i].abs();
        }

        state[0]
    }
}

impl<'a> RescueHash<'a> {
    pub fn new(
        finite_field: &'a FiniteField,
        rate: usize,
        capacity: usize,
        alpha: FieldElement<'a>,
        mds_matrix: Array2<FieldElement<'a>>,
        constants: Array1<FieldElement<'a>>,
    ) -> Self {
        assert_ne!(
            (finite_field.prime - 1) % alpha.value(),
            0,
            "Alpha should not divide p-1"
        );
        let alpha_inv = alpha.inverse();

        Self {
            alpha,
            alpha_inv,
            finite_field,
            rate,
            capacity,
            mds_matrix,
            constants,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::{Hasher, RescueHash};
    use algebra::finite_field::FiniteField;
    use ndarray::{array, Array1};

    #[test]
    fn test_new() {
        let finite_field = FiniteField::new(97, 1);
        let alpha = finite_field.element(5);
        let mds_matrix = array![
            [finite_field.random_element(), finite_field.random_element()],
            [finite_field.random_element(), finite_field.random_element()],
        ];
        let constants = Array1::from_elem(108, finite_field.random_element());
        let hash_func = RescueHash::new(&finite_field, 1, 1, alpha, mds_matrix, constants);
        let hash = hash_func.hash(finite_field.element(15));

        println!("Hash: {}", hash);
    }
}
