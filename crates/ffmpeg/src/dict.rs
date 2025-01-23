use std::collections::HashMap;
use std::ffi::{CStr, CString};

use ffmpeg_sys_next::*;

use crate::error::FfmpegError;
use crate::smart_object::SmartPtr;

pub struct Dictionary {
    ptr: SmartPtr<AVDictionary>,
}

/// Safety: `Dictionary` is safe to send between threads.
unsafe impl Send for Dictionary {}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dict = HashMap::<String, String>::from(self);
        dict.fmt(f)
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        let mut dict = Self::new();

        Self::clone_from(&mut dict, self);

        dict
    }

    fn clone_from(&mut self, source: &Self) {
        // Safety: av_dict_copy is safe to call
        let ret = unsafe { av_dict_copy(self.as_mut_ptr_ref(), source.as_ptr(), 0) };
        if ret < 0 {
            panic!("failed to clone dictionary: {ret}")
        }
    }
}

pub struct DictionaryBuilder {
    dict: Dictionary,
}

impl DictionaryBuilder {
    pub fn set(mut self, key: &str, value: &str) -> Self {
        self.dict.set(key, value).expect("Failed to set dictionary entry");
        self
    }

    pub fn build(self) -> Dictionary {
        self.dict
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            // Safety: A null pointer is a valid dictionary, and a valid pointer.
            ptr: unsafe {
                SmartPtr::wrap(std::ptr::null_mut(), |ptr| {
                    // Safety: av_dict_free is safe to call
                    av_dict_free(ptr)
                })
            },
        }
    }

    pub fn builder() -> DictionaryBuilder {
        DictionaryBuilder { dict: Self::new() }
    }

    /// # Safety
    /// `ptr` must be a valid pointer.
    /// The caller must also ensure that the dictionary is not freed while this
    /// object is alive, and that we don't use the pointer as mutable
    pub unsafe fn from_ptr_ref(ptr: *mut AVDictionary) -> Self {
        // We don't own the dictionary, so we don't need to free it
        Self {
            ptr: SmartPtr::wrap(ptr as _, |_| {}),
        }
    }

    /// # Safety
    /// `ptr` must be a valid pointer.
    pub unsafe fn from_ptr_owned(ptr: *mut AVDictionary) -> Self {
        Self {
            ptr: SmartPtr::wrap(ptr, |ptr| {
                // Safety: av_dict_free is safe to call
                av_dict_free(ptr)
            }),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), FfmpegError> {
        if key.is_empty() {
            return Err(FfmpegError::Arguments("Keys cannot be empty"));
        }
        let key = CString::new(key).expect("Failed to convert key to CString");
        let value = CString::new(value).expect("Failed to convert value to CString");

        // Safety: av_dict_set is safe to call
        let ret = unsafe { av_dict_set(self.ptr.as_mut(), key.as_ptr(), value.as_ptr(), 0) };

        if ret < 0 {
            Err(FfmpegError::Code(ret.into()))
        } else {
            Ok(())
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if key.is_empty() {
            return None;
        }
        let key = CString::new(key).expect("Failed to convert key to CString");

        // Safety: av_dict_get is safe to call
        let entry = unsafe { av_dict_get(self.as_ptr(), key.as_ptr(), std::ptr::null_mut(), AV_DICT_IGNORE_SUFFIX) };

        if entry.is_null() {
            None
        } else {
            // Safety: av_dict_get is safe to call
            Some(unsafe { CStr::from_ptr((*entry).value) }.to_string_lossy().into_owned())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    pub fn iter(&self) -> DictionaryIterator {
        DictionaryIterator::new(self)
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr_ref(&mut self) -> &mut *mut AVDictionary {
        self.ptr.as_mut()
    }

    pub fn into_ptr(self) -> *mut AVDictionary {
        self.ptr.into_inner()
    }
}

pub struct DictionaryIterator<'a> {
    dict: &'a Dictionary,
    entry: *mut AVDictionaryEntry,
}

impl<'a> DictionaryIterator<'a> {
    pub fn new(dict: &'a Dictionary) -> Self {
        Self {
            dict,
            entry: std::ptr::null_mut(),
        }
    }
}

impl<'a> Iterator for DictionaryIterator<'a> {
    type Item = (&'a CStr, &'a CStr);

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: av_dict_get is safe to call
        self.entry = unsafe { av_dict_get(self.dict.as_ptr(), &[0] as *const _ as _, self.entry, AV_DICT_IGNORE_SUFFIX) };

        if self.entry.is_null() {
            None
        } else {
            // Safety: av_dict_get is safe to call
            let key = unsafe { CStr::from_ptr((*self.entry).key) };
            // Safety: av_dict_get is safe to call
            let value = unsafe { CStr::from_ptr((*self.entry).value) };

            Some((key, value))
        }
    }
}

impl<'a> IntoIterator for &'a Dictionary {
    type IntoIter = DictionaryIterator<'a>;
    type Item = <DictionaryIterator<'a> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        DictionaryIterator::new(self)
    }
}

impl From<HashMap<String, String>> for Dictionary {
    fn from(map: HashMap<String, String>) -> Self {
        let mut dict = Dictionary::new();

        for (key, value) in map {
            if key.is_empty() || value.is_empty() {
                continue;
            }

            dict.set(&key, &value).expect("Failed to set dictionary entry");
        }

        dict
    }
}

impl From<&Dictionary> for HashMap<String, String> {
    fn from(dict: &Dictionary) -> Self {
        dict.into_iter()
            .map(|(key, value)| (key.to_string_lossy().into_owned(), value.to_string_lossy().into_owned()))
            .collect()
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {

    use std::collections::HashMap;

    use crate::dict::Dictionary;

    fn sort_hashmap<K: Ord, V>(map: std::collections::HashMap<K, V>) -> std::collections::BTreeMap<K, V> {
        map.into_iter().collect()
    }

    #[test]
    fn test_dict_default_and_items() {
        let mut dict = Dictionary::default();

        assert!(dict.is_empty(), "Default dictionary should be empty");
        assert!(dict.as_ptr().is_null(), "Default dictionary pointer should be null");

        dict.set("key1", "value1").expect("Failed to set key1");
        dict.set("key2", "value2").expect("Failed to set key2");
        dict.set("key3", "value3").expect("Failed to set key3");

        let dict_hm: std::collections::HashMap<String, String> = HashMap::from(&dict);

        insta::assert_debug_snapshot!(sort_hashmap(dict_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
        "#);
    }

    #[test]
    fn test_dict_set_empty_key() {
        let mut dict = Dictionary::new();
        assert!(dict.set("", "value1").is_err());
    }

    #[test]
    fn test_dict_clone_empty() {
        let empty_dict = Dictionary::new();
        let cloned_dict = empty_dict.clone();

        assert!(cloned_dict.is_empty(), "Cloned dictionary should be empty");
        assert!(empty_dict.is_empty(), "Original dictionary should remain empty");
    }

    #[test]
    fn test_dict_clone_non_empty() {
        let mut dict = Dictionary::new();
        dict.set("key1", "value1").expect("Failed to set key1");
        dict.set("key2", "value2").expect("Failed to set key2");
        let mut clone = dict.clone();

        let dict_hm: std::collections::HashMap<String, String> = HashMap::from(&dict);
        let clone_hm: std::collections::HashMap<String, String> = HashMap::from(&clone);

        insta::assert_debug_snapshot!(sort_hashmap(dict_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
        }
        "#);
        insta::assert_debug_snapshot!(sort_hashmap(clone_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
        }
        "#);

        clone.set("key3", "value3").expect("Failed to set key3 in cloned dictionary");

        let dict_hm: std::collections::HashMap<String, String> = HashMap::from(&dict);
        let clone_hm: std::collections::HashMap<String, String> = HashMap::from(&clone);
        insta::assert_debug_snapshot!(sort_hashmap(dict_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
        }
        "#);
        insta::assert_debug_snapshot!(sort_hashmap(clone_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
        "#);
    }

    #[test]
    fn test_dictionary_builder_set_and_build() {
        let dict = Dictionary::builder()
            .set("key1", "value1")
            .set("key2", "value2")
            .set("key3", "value3")
            .build();

        let dict_hm: std::collections::HashMap<String, String> = HashMap::from(&dict);
        insta::assert_debug_snapshot!(sort_hashmap(dict_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
        "#);
    }

    #[test]
    fn test_dict_get() {
        let mut dict = Dictionary::new();
        assert!(
            dict.get("nonexistent_key").is_none(),
            "Getting a nonexistent key from an empty dictionary should return None"
        );

        dict.set("key1", "value1").expect("Failed to set key1");
        dict.set("key2", "value2").expect("Failed to set key2");
        assert_eq!(
            dict.get("key1"),
            Some("value1".to_string()),
            "The value for 'key1' should be 'value1'"
        );
        assert_eq!(
            dict.get("key2"),
            Some("value2".to_string()),
            "The value for 'key2' should be 'value2'"
        );

        assert!(dict.get("key3").is_none(), "Getting a nonexistent key should return None");

        dict.set("special_key!", "special_value").expect("Failed to set special_key!");
        assert_eq!(
            dict.get("special_key!"),
            Some("special_value".to_string()),
            "The value for 'special_key!' should be 'special_value'"
        );

        dbg!(dict.get(""));
        assert!(
            dict.get("").is_none(),
            "Getting an empty key should return None (empty keys are not allowed)"
        );
    }

    #[test]
    fn test_from_hashmap_for_dictionary() {
        let mut hash_map = std::collections::HashMap::new();
        hash_map.insert("key1".to_string(), "value1".to_string());
        hash_map.insert("key2".to_string(), "value2".to_string());
        hash_map.insert("key3".to_string(), "value3".to_string());
        let dict = Dictionary::from(hash_map);

        let dict_hm: std::collections::HashMap<String, String> = HashMap::from(&dict);
        insta::assert_debug_snapshot!(sort_hashmap(dict_hm), @r#"
        {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
        "#);

        let mut hash_map_with_empty = std::collections::HashMap::new();
        hash_map_with_empty.insert("key1".to_string(), "value1".to_string());
        hash_map_with_empty.insert("".to_string(), "value2".to_string());
        hash_map_with_empty.insert("key3".to_string(), "".to_string());

        let dict_with_empty = Dictionary::from(hash_map_with_empty);

        let dict_with_empty_hm: std::collections::HashMap<String, String> = HashMap::from(&dict_with_empty);
        insta::assert_debug_snapshot!(sort_hashmap(dict_with_empty_hm), @r#"
        {
            "key1": "value1",
        }
        "#);
    }
}
