use crate::hash::Hasher;
use algebra::finite_field::{FieldElement, FiniteField};
use std::rc::Rc;

struct MerkleTree<H: Hasher> {
    finite_field: Rc<FiniteField>,
    hasher: H,
}

impl<H: Hasher> MerkleTree<H> {
    /// computes the Merkle root of a given array.
    pub fn new(finite_field: Rc<FiniteField>, hasher: H) -> Self {
        MerkleTree {
            finite_field,
            hasher,
        }
    }

    fn commit_inner(&self, leafs: &[FieldElement]) -> FieldElement {
        let leafs_len = leafs.len();

        return if leafs_len == 1 {
            leafs.first().unwrap().to_owned()
        } else {
            let left = &leafs[..leafs_len / 2];
            let right = &leafs[leafs_len / 2..leafs_len];
            self.hasher.hash(self.commit(left) + self.commit(right))
        };
    }
    pub fn commit(&self, leafs: &[FieldElement]) -> FieldElement {
        let leafs_len = leafs.len();
        assert_ne!(leafs_len, 0, "The list doesn't contains any elements");
        assert_eq!(
            (leafs_len & (leafs_len - 1)),
            0,
            "The list is not power of 2"
        );

        let hashed_leafs = leafs
            .into_iter()
            .map(|leaf| self.hasher.hash(leaf.clone()))
            .collect::<Vec<FieldElement>>();
        return self.commit_inner(&hashed_leafs);
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
            auth_path.push(self.commit(&leafs[half_leafs_len..]));
            auth_path.to_owned()
        } else {
            let mut auth_path = self.open(index - half_leafs_len, &leafs[half_leafs_len..]);
            auth_path.push(self.commit(&leafs[..half_leafs_len]));
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
