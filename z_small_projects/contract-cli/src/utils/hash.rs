use sha2::{Sha512, Digest};

/// Converts an owner string to a SHA-512 hex string
pub fn hash_owner(owner: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(owner.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_consistently() {
        let a = hash_owner("alice");
        let b = hash_owner("alice");
        assert_eq!(a, b);
        assert_eq!(a.len(), 128); // SHA-512 in hex = 128 chars
    }
}
