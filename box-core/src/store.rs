use std::hash::BuildHasher;

use rapidhash::{HashMapExt, RapidHashMap};

use crate::BoxVariant;

/// Global store for box computations
#[derive(Debug, Clone)]
pub struct BoxStore {
    /// Store boxes by their hash
    pub boxes: RapidHashMap<u64, BoxVariant>,
    /// Look up table for variable names
    pub vars: RapidHashMap<u64, String>,
    /// Look up table for variable names
    pub rev_vars: RapidHashMap<String, u64>,
}

impl Default for BoxStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxStore {
    /// Initialize a new store
    pub fn new() -> Self {
        let boxes = RapidHashMap::new();
        let vars = RapidHashMap::new();
        let rev_vars = RapidHashMap::new();

        Self {
            boxes,
            vars,
            rev_vars,
        }
    }

    // Store a box by its hash
    pub fn store(&mut self, value: BoxVariant) {
        let hash = self.boxes.hasher().hash_one(&value);
        self.boxes.insert(hash, value);
    }

    /// Store a box and bind it to a variable name
    pub fn store_with_name(&mut self, name: impl Into<String>, value: impl Into<BoxVariant>) {
        let value = value.into();
        let hash = self.boxes.hasher().hash_one(&value);
        self.boxes.insert(hash, value);
        let name = name.into();
        self.vars.insert(hash, name.clone());
        self.rev_vars.insert(name, hash);
    }

    /// Fetch a name by its hash
    pub fn fetch_name(&self, hash: u64) -> Option<String> {
        let result = self.vars.get(&hash)?;
        Some(result.clone())
    }

    /// Fetch a box by its hash
    pub fn fetch_box(&self, hash: u64) -> Option<BoxVariant> {
        self.boxes.get(&hash).cloned()
    }

    /// Fetch a box by its hash
    pub fn fetch_box_by_name(&self, name: &str) -> Option<BoxVariant> {
        let res = self.rev_vars.get(name);
        if let Some(&hash) = res {
            return self.fetch_box(hash);
        }
        None
    }
}
