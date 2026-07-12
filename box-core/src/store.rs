use std::hash::BuildHasher;

use rapidhash::{HashMapExt, RapidHashMap};

use crate::BoxVariant;

/// Global store for box computations
#[derive(Debug)]
pub struct BoxStore {
    /// Store boxes by their hash
    pub boxes: RapidHashMap<u64, BoxVariant>,
    /// Look up table for variable names
    pub variables: RapidHashMap<String, u64>,
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
        let variables = RapidHashMap::new();

        Self { boxes, variables }
    }

    // Store a box by its hash
    pub fn store_box(&mut self, value: BoxVariant) {
        let hash = self.boxes.hasher().hash_one(&value);
        self.boxes.insert(hash, value);
    }

    /// Store a box and bind it to a variable name
    pub fn store_box_with_name(&mut self, name: impl Into<String>, value: impl Into<BoxVariant>) {
        let value = value.into();
        let hash = self.boxes.hasher().hash_one(&value);
        self.variables.insert(name.into(), hash);
        self.boxes.insert(hash, value);
    }

    /// Fetch a box from the store by its name
    pub fn fetch_box_by_name(&self, name: &str) -> Option<BoxVariant> {
        let hash = self.variables.get(name)?;
        self.boxes.get(hash).cloned()
    }

    /// Fetch a box from the store by its hash
    pub fn fetch_box_by_hash(&self, hash: u64) -> Option<BoxVariant> {
        self.boxes.get(&hash).cloned()
    }
}
