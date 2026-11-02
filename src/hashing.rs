use sha2::{Digest, Sha256};


pub fn hash(data: &[u8]) -> [u8; 32] {
    let hash_result = Sha256::digest(data);
    hash_result.try_into().unwrap()
}

pub fn hash_leaf(data: &[u8]) -> [u8; 32] {
    hash([[0x00].as_slice(),data].concat().as_slice())
}

pub fn hash_internal(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    hash([[0x01].as_slice(),left.as_slice(), right.as_slice()].concat().as_slice())
}

pub fn generate_default_hashes(depth: usize) -> Vec<[u8; 32]> {
    let mut default_hashes = Vec::new();
    default_hashes.push(hash_leaf(&[]));
    for i in 0..depth {
        let previous = default_hashes[i];
        default_hashes.push(hash_internal(&previous, &previous));
    }
    default_hashes
}

#[cfg(test)]

mod test{
    use std::{assert_eq, assert_ne};

use super::* ;
    #[test]
    fn testhash() {
        assert_eq!(hash_leaf("Alice".as_bytes()).len(), 32);
        assert_eq!(hash_leaf("Alice".as_bytes()), hash_leaf("Alice".as_bytes()));
        assert_ne!(hash_leaf("Alice".as_bytes()), hash_leaf("Bob".as_bytes()));
        assert_eq!(hash_internal(&hash_leaf("Alice".as_bytes()), &hash_leaf("Bob".as_bytes())).len(), 32);
        assert_ne!(hash_internal(&hash_leaf("Alice".as_bytes()), &hash_leaf("Bob".as_bytes())), hash_internal(&hash_leaf("Bob".as_bytes()), &hash_leaf("Alice".as_bytes())))
    }

    #[test]
    fn test_default_hashes() {
        assert_eq!(generate_default_hashes(4).len(), 5);
        let default_hash = generate_default_hashes(4);
        for i in default_hash {
            assert_eq!(i.len(), 32);
        }
        let default_hash = generate_default_hashes(4);
        for i in 1..5 {
            assert_ne!(default_hash.get(i), default_hash.get(i - 1));
        }
        let default_hash = generate_default_hashes(4);
        let default_hash2 = generate_default_hashes(4);
        for i in 0..5 {
            assert_eq!(default_hash.get(i), default_hash2.get(i));
        }
    }

}