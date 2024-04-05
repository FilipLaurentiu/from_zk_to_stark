use algebra::finite_field::FieldElement;

pub trait Hasher {
    fn hash(&self, value: FieldElement) -> FieldElement;
}

struct RescueHash {}

impl Hasher for RescueHash {
    fn hash(&self, value: FieldElement) -> FieldElement {
        todo!()
    }
}
