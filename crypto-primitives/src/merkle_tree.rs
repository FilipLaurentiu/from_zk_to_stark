use crate::hash::Hasher;
use algebra::finite_field::{FieldElement, FiniteField};
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

        let tree = MerkleTree {
            finite_field,
            hasher: hasher.clone(),
            leafs: leafs
                .iter()
                .map(|leaf| hasher.hash(leaf.clone()))
                .collect::<Vec<FieldElement>>(),
            levels: vec![],
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
    pub fn open(&self, index: usize, leafs: &[FieldElement]) -> Vec<FieldElement> {
        let leafs_len = leafs.len();
        assert!(leafs_len != 0 && (leafs_len & (leafs_len - 1)) == 0);
        assert!(index < leafs_len);

        let half_leafs_len = leafs_len / 2;
        let is_two_leafs = leafs_len == 2;
        let index_is_less_than_half = index < half_leafs_len;

        if is_two_leafs {
            vec![leafs[leafs_len - index - 1].clone()]
        } else if index_is_less_than_half {
            let mut auth_path = self.open(index, &leafs[..half_leafs_len]);
            //auth_path.push(self.commit(&leafs[half_leafs_len..]));
            auth_path.to_owned()
        } else {
            let mut auth_path = self.open(index - half_leafs_len, &leafs[half_leafs_len..]);
            // auth_path.push(self.commit(&leafs[..half_leafs_len]));
            auth_path.to_owned()
        }
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
    use crate::hash::RescueHash;
    use crate::merkle_tree::MerkleTree;
    use algebra::finite_field::FiniteField;
    use std::rc::Rc;

    #[test]
    fn test_create_merkle_tree() {
        let finite_field = Rc::new(FiniteField::new(97, 1));
        let hasher = RescueHash::default();
        let leafs = vec![
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
            finite_field.random_element(),
        ];
        let mut tree = MerkleTree::new(Rc::clone(&finite_field), hasher, leafs);
        let root = tree.commit();
        assert_eq!(tree.levels.len(), tree.leafs.len().ilog2() as usize);
        println!("Root: {}", root);
    }
}
