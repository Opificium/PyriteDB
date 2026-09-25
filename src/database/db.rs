use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;


/// Errors, which can occur while accessing DB.
/// thiserror generates 'display' implementation from the
/// '#[error("...")]' attributes, as well as 'std:error:Error', via derive macro
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Read access has failed: Lock poisoned (Other Thread with lock on hold has crashed)")]
    ReadLockPoisoned,

    #[error("Write access has failed: Lock poisoned (Other Thread with lock on hold has crashed)")]
    WriteLockPoisoned,
}

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
    pub fn get(&self, key: &str) -> Result<Option<String>, DbError> {
        let map = self.inner.read().map_err(|_| DbError::ReadLockPoisoned)?;
        Ok(map.get(key).cloned())
    }

    /// Set value in DB
    pub fn set(&self, key: String, value: String) -> Result<(), DbError> {
        let mut map = self.inner.write().map_err(|_| DbError::WriteLockPoisoned)?;
        map.insert(key, value);
        Ok(())
    }

    /// Delete value/row from DB by key
    /// Returns Ok(true), if key exists and has been removed, Ok(false) else
    pub fn del(&self, key: &str) -> Result<bool, DbError> {
        let mut map = self.inner.write().map_err(|_| DbError::WriteLockPoisoned)?;
        Ok(map.remove(key).is_some())
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
        db.set("foo".to_string(), "bar".to_string()).unwrap();
        assert_eq!(db.get("foo").unwrap(), Some("bar".to_string()));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let db = Db::new();
        assert_eq!(db.get("missing").unwrap(), None);
    }

    #[test]
    fn del_existing_key_returns_true() {
        let db = Db::new();
        db.set("foo".to_string(), "bar".to_string()).unwrap();
        assert!(db.del("foo").unwrap());
        assert_eq!(db.get("foo").unwrap(), None);
    }

    #[test]
    fn del_missing_key_returns_false() {
        let db = Db::new();
        assert!(!db.del("missing").unwrap());
    }
}