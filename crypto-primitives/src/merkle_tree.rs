use crate::hash::Hasher;
use algebra::finite_field::{FieldElement, FiniteField};
use std::ops::Index;
use std::rc::Rc;

struct MerkleTree<H: Hasher + Clone> {
    finite_field: Rc<FiniteField>,
    hasher: H,
    leafs: Vec<FieldElement>,
    levels: Vec<Vec<FieldElement>>,
}

impl<H: Hasher + Clone> MerkleTree<H> {
    /// computes the Merkle root of a given array.
    pub fn new(finite_field: Rc<FiniteField>, hasher: H, leafs: Vec<FieldElement>) -> Self {
        let leafs_len = leafs.len();
        assert_ne!(leafs_len, 0, "The list doesn't contains any elements");
        assert_eq!(leafs_len & (leafs_len - 1), 0, "The list is not power of 2");

        let leafs = leafs
            .iter()
            .map(|leaf| hasher.hash(leaf.clone()))
            .collect::<Vec<FieldElement>>();

        let tree = MerkleTree {
            finite_field,
            hasher: hasher.clone(),
            leafs: leafs.clone(),
            levels: vec![leafs],
        };
        tree
    }

    pub fn commit(&mut self) -> FieldElement {
        let mut curr_level = self.leafs.clone();

        while curr_level.len() > 1 {
            let odd_leafs = curr_level
                .clone()
                .into_iter()
                .step_by(2)
                .collect::<Vec<FieldElement>>();
            let even_leafs = curr_level
                .clone()
                .into_iter()
                .skip(1)
                .step_by(2)
                .collect::<Vec<FieldElement>>();

            let parents = odd_leafs
                .iter()
                .zip(even_leafs.iter())
                .map(|(left, right)| self.hasher.hash(left + right))
                .collect::<Vec<FieldElement>>();
            self.levels.push(parents.clone());
            curr_level = parents;
        }

        curr_level.first().unwrap().clone()
    }

    /// computes the authentication path of an indicated leaf in the Merkle tree.
    pub fn prove(&self, element: FieldElement) -> Option<Vec<FieldElement>> {
        let mut current_level_index = 0usize;

        let mut result: Vec<FieldElement> = vec![];
        let mut element = element;

        while current_level_index < self.levels.len() {
            let current_level = &self.levels[current_level_index];

            if current_level.len() == 1 {
                assert_eq!(element, current_level.first().unwrap().clone());
                result.push(element.clone());
                return Some(result);
            } else {
                match current_level.iter().position(|x| *x == element) {
                    Some(element_index) => {
                        let sibling;
                        if element_index % 2 == 0 {
                            sibling = current_level.index(element_index + 1);
                            result.push(element.clone());
                            result.push(sibling.clone());
                        } else {
                            sibling = current_level.index(element_index - 1);
                            result.push(sibling.clone());
                            result.push(element.clone());
                        };
                        element = self.hasher.hash(sibling.clone() + element);

                        current_level_index += 1;
                    }

                    None => {
                        return None;
                    }
                }
            }
        }

        Some(result)
    }

    ///  verifies that a given leaf is an element of the committed vector at the given index
    pub fn verify(
        &self,
        root: &FieldElement,
        index: usize,
        path: &[FieldElement],
        leaf: FieldElement,
    ) -> bool {
        assert!(index < (1 << path.len()));

        if path.len() == 1 {
            if index == 0 {
                return root == &self.hasher.hash(leaf + path[0].clone());
            } else {
                return root == &self.hasher.hash(path[0].clone() + leaf);
            }
        } else {
            if index % 2 == 0 {
                return self.verify(
                    root,
                    index >> 1,
                    &path[1..],
                    self.hasher.hash(leaf + path[0].clone()),
                );
            } else {
                return self.verify(
                    root,
                    index >> 1,
                    &path[1..],
                    self.hasher.hash(path[0].clone() + leaf),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::{Hasher, RescueHash};
    use crate::merkle_tree::MerkleTree;
    use algebra::finite_field::FiniteField;
    use std::rc::Rc;

    #[test]
    fn test_create_merkle_tree() {
        let finite_field = Rc::new(FiniteField::new(97, 1));
        let hasher = RescueHash::default();

        let element = finite_field.random_element();
        let leafs = vec![
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            element.clone(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
        ];
        let mut tree = MerkleTree::new(Rc::clone(&finite_field), hasher.clone(), leafs);
        let root = tree.commit();
        assert_eq!(tree.levels.len(), tree.leafs.len().ilog2() as usize + 1);
        println!("Root: {}", root);

        let element_hash = hasher.hash(element);
        let proof = tree.prove(element_hash);
        println!("Proof: {:?}", proof);
        assert!(proof.is_some());
    }
}
