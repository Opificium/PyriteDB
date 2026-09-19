use std::collections::HashMap;
use std::sync::RwLock;

/// Encapsulates DB-state behind API.
/// Visible outside are only get/set/del
pub struct Db {
    inner: RwLock<HashMap<String, String>>
}

impl Db {
    /// Create new instance of DB
    pub fn new() -> Self {
        Db {
            inner: RwLock::new(HashMap::new())
        }
    }

    /// Get value from DB
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner
            .read()
            .expect("RwLock poisoned")
            .get(key)
            .cloned()
    }

    /// Set value in DB
    pub fn set(&self, key: String, value: String) {
        self.inner
            .write()
            .expect("RwLock poisoned")
            .insert(key, value);
    }

    /// Delete value/row from DB by key
    pub fn del(&self, key: &str) -> bool {
        self.inner
            .write()
            .expect("RwLock poisoned")
            .remove(key)
            .is_some()
    }
}

// Default implementation, since new() does not need any arguments
impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_then_get_returns_value() {
        let db = Db::new();
        db.set("foo".to_string(), "bar".to_string());
        assert_eq!(db.get("foo"), Some("bar".to_string()));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let db = Db::new();
        assert_eq!(db.get("missing"), None);
    }

    #[test]
    fn del_existing_key_returns_true() {
        let db = Db::new();
        db.set("foo".to_string(), "bar".to_string());
        assert!(db.del("foo"));
        assert_eq!(db.get("foo"), None);
    }

    #[test]
    fn del_missing_key_returns_false() {
        let db = Db::new();
        assert!(!db.del("missing"));
    }
}