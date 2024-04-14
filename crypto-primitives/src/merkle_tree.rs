use crate::hash::Hasher;
use algebra::finite_field::{FieldElement, FiniteField};

struct MerkleTree<'a, H: Hasher<'a>> {
    finite_field: &'a FiniteField,
    hasher: &'a H,
}

impl<'a, H: Hasher<'a>> MerkleTree<'a, H> {
    /// computes the Merkle root of a given array.
    pub fn new(finite_field: &'a FiniteField, hasher: &'a H) -> Self {
        MerkleTree {
            finite_field,
            hasher,
        }
    }
    pub fn commit(&self, leafs: &'a [FieldElement<'a>]) -> FieldElement<'a> {
        let leafs_len = leafs.len();
        assert!(leafs_len != 0 && (leafs_len & (leafs_len - 1)) == 0);
        if leafs_len == 1 {
            return leafs.first().unwrap().to_owned();
        } else {
            let left = &leafs[..leafs_len / 2];
            let right = &leafs[leafs_len / 2..leafs_len];
            return self.hasher.hash(self.commit(left) + self.commit(right));
        }
    }

    /// computes the authentication path of an indicated leaf in the Merkle tree.
    pub fn open(&self, index: usize, leafs: &'a [FieldElement<'a>]) -> Vec<FieldElement> {
        let leafs_len = leafs.len();
        assert!(leafs_len != 0 && (leafs_len & (leafs_len - 1)) == 0);
        assert!(index < leafs_len);

        let half_leafs_len = leafs_len / 2;
        let is_two_leafs = leafs_len == 2;
        let index_is_less_than_half = index < half_leafs_len;

        if is_two_leafs {
            vec![leafs[leafs_len - index - 1]]
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
        root: &FieldElement<'a>,
        index: usize,
        path: &[FieldElement<'a>],
        leaf: FieldElement<'a>,
    ) -> bool {
        assert!(index < (1 << path.len()));

        if path.len() == 1 {
            if index == 0 {
                return root == &self.hasher.hash(leaf + path[0]);
            } else {
                return root == &self.hasher.hash(path[0] + leaf);
            }
        } else {
            if index % 2 == 0 {
                return self.verify(
                    root,
                    index >> 1,
                    &path[1..],
                    self.hasher.hash(leaf + path[0]),
                );
            } else {
                return self.verify(
                    root,
                    index >> 1,
                    &path[1..],
                    self.hasher.hash(path[0] + leaf),
                );
            }
        }
    }
}
