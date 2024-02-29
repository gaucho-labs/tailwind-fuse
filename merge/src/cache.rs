use std::num::NonZeroUsize;

use lru::LruCache;

#[derive(Debug, Clone)]
pub struct MergeCache {
    cache: LruCache<String, Option<String>>,
}

impl MergeCache {
    pub fn new(size: NonZeroUsize) -> Self {
        MergeCache {
            cache: LruCache::new(size),
        }
    }
}

impl MergeCache {
    pub fn merge(&mut self, class: String) -> String {
        if let Some(value) = self.cache.get_mut(&class) {
            value.as_ref().cloned().unwrap_or(class)
        } else {
            let merged = crate::merge::tw_merge(&class);
            self.cache.put(class.clone(), merged.clone());
            merged.unwrap_or(class)
        }
    }
}
