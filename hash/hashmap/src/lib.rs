use std::{
    borrow,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter::FromIterator,
};
use std::{mem, ops};

const INITIAL_NBUCKETS: usize = 1;

#[derive(Debug)]
pub struct HashMap<K, V> {
    // each bucket is a Vec of (key, value) pairs
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq,
{
    pub fn bucket<Q>(&self, key: &Q) -> usize
    where
        K: borrow::Borrow<Q>,
        Q: Hash + PartialEq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        // Drain all elements from buckets and insert them into new_buckets
        for (key, value) in self.buckets.drain(..).flatten() {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }
        mem::swap(&mut self.buckets, &mut new_buckets);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // If buckets is empty or if map is 3/4th full
        if self.buckets.is_empty() || self.items > self.buckets.len() * 3 / 4 {
            self.resize();
        }
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.items += 1;
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                // return existing value
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        None
    }

    /// Get ideally takes a reference to a key and returns a reference to the value
    /// But in this case, it should take a 'Q' where 'K' can be borrowed as 'Q'
    /// Q is Hash + PartialEq because K is Hash + PartialEq. Q is also ?Sized
    /// so that the hashmap can support types such as &str as keys which is unsized
    /// Now 'ekey' is of type 'K' and 'key' is of type 'Q'. So borrow 'ekey' as'Q'
    /// in order to compare them.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: borrow::Borrow<Q>,
        Q: Hash + PartialEq + ?Sized,
    {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|(ref ekey, _)| ekey.borrow() == key)
            .map(|(_, ref v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: borrow::Borrow<Q>,
        Q: Hash + PartialEq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: borrow::Borrow<Q>,
        Q: Hash + PartialEq + ?Sized,
    {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket
            .iter()
            .position(|(ref ekey, _)| ekey.borrow() == key)?;
        // use swap_remove to avoid shifting elements
        let val = bucket.swap_remove(i).1;
        self.items -= 1;
        Some(val)
    }

    /// return the number of elements in the map
    pub fn len(&self) -> usize {
        self.items
    }

    /// return true if the map is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Entry API gives a mutable reference to where something that is
/// inserted in the map. Entry is an enum with two variants:
/// 1. Vacant - when the key is not present in the map
/// 2. Occupied - when the key is present in the map
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

/// OccupiedEntry is returned when the key is present in the map
/// It contains a mutable reference to the entry in the vector
pub struct OccupiedEntry<'a, K, V> {
    entry: &'a mut (K, V),
}

/// VacantEntry is returned when the key is not present in the map
/// It contains a mutable reference to the map and the bucket where
/// the element is inserted. It will also keep the key to be inserted
/// because the key needed when the entry is inserted later.
pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    map: &'a mut HashMap<K, V>,
    bucket: usize,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    /// Consumes self and inserts the key-value pair into the map
    /// Returns a mutable reference to the value so that it can be
    /// modified later as part of the entry API where this is used.
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + PartialEq,
    {
        self.map.buckets[self.bucket].push((self.key, value));
        self.map.items += 1;
        // unwrap is safe because we just inserted the value
        &mut self.map.buckets[self.bucket].last_mut().unwrap().1
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + PartialEq,
{
    /// Consumes self and returns a mutable reference to the value
    /// so that it can be modified later. This helps in updating the
    /// value in the map regardless of whether the key is present or not.
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => &mut entry.entry.1,
            Entry::Vacant(entry) => entry.insert(value),
        }
    }

    /// Similar to or_insert but takes a maker F where F is a closure
    /// that returns a V. The function itself returns a reference to the
    /// value being inserted. This is useful when the value is expensive
    /// to create and should only be created if the key is not present
    /// in the map. In this case, 'maker()' is only called if the key is
    /// not present in the map.
    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => &mut entry.entry.1,
            Entry::Vacant(entry) => entry.insert(maker()),
        }
    }

    /// Similar to or_insert_with but creates a default value using the
    /// Default trait. In this case, the value type V must implement the
    /// Default trait.
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq,
{
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        if self.buckets.is_empty() || self.items > self.buckets.len() * 3 / 4 {
            self.resize();
        }
        let bucket = self.bucket(&key);
        // Find an element in the bucket that matches the key and return
        // mutable reference to it if found. If not found, return a Vacant
        // entry with the key and mutable reference to the bucket.
        match self.buckets[bucket]
            .iter_mut()
            .find(|(ref ekey, _)| ekey == &key)
        {
            Some(entry) => Entry::Occupied(OccupiedEntry {
                // use unsafe to avoid borrowing issues
                entry: unsafe { mem::transmute(entry) },
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                map: self,
                bucket,
            }),
        }
    }
}

impl<K, Q: ?Sized, V> ops::Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + borrow::Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

/// Iterator for HashMap
pub struct HashMapIter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> HashMapIter<'a, K, V> {
    fn new(map: &'a HashMap<K, V>) -> Self {
        Self {
            map,
            bucket: 0,
            at: 0,
        }
    }
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // Loop is required since the bucket list can be sparse
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.at) {
                    Some((ref k, ref v)) => {
                        self.at += 1;
                        return Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => {
                    return None;
                }
            }
        }
    }
}

/// Implement IntoIterator for HashMap to allow iterating over
/// the map using a reference to the map.
impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIter::new(self)
    }
}

/// Owned iterator for HashMap
pub struct HashMapIntoIter<K, V> {
    map: HashMap<K, V>,
    bucket: usize,
}

impl<K, V> Iterator for HashMapIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(bucket) = self.map.buckets.get_mut(self.bucket) {
            if !bucket.is_empty() {
                return Some(bucket.swap_remove(0));
            }
            // Move to the next bucket if the current one is empty.
            self.bucket += 1;
        }
        None
    }
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = HashMapIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIntoIter {
            map: self,
            bucket: 0,
        }
    }
}

/// Implement FromIterator for HashMap
/// This allows us to collect an iterator of key-value pairs into a HashMap
/// The from_iter takes a generic iterator that produces (K, V) pairs.
impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hash + PartialEq,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut map = HashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", 42), None);
        assert_eq!(map.insert("foo", 23), Some(42));
    }

    #[test]
    fn test_get() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.get(&"bar"), None);
    }
    #[test]
    fn test_contains_key() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert!(map.contains_key(&"foo"));
        assert!(!map.contains_key(&"bar"));
    }

    #[test]
    fn test_remove() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"bar"), None);
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.remove(&"foo"), None);
        assert_eq!(map.get(&"foo"), None);
    }
    #[test]
    fn test_len() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        map.insert("bar", 23);
        assert_eq!(map.len(), 2);
        map.remove(&"foo");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        map.remove(&"bar");
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }
}

#[test]
fn test_iterator() {
    let mut map = HashMap::new();
    map.insert("foo", 42);
    map.insert("bar", 23);
    map.insert("baz", 142);
    map.insert("quox", 7);
    for (&k, &v) in &map {
        match k {
            "foo" => assert_eq!(v, 42),
            "bar" => assert_eq!(v, 23),
            "baz" => assert_eq!(v, 142),
            "quox" => assert_eq!(v, 7),
            _ => unreachable!(),
        }
    }
    assert_eq!(map.into_iter().count(), 4);
}

#[test]
fn test_into_iterator() {
    let mut map = HashMap::new();
    map.insert("foo", 42);
    map.insert("bar", 23);
    map.insert("baz", 142);
    map.insert("quox", 7);
    let mut items = 0;
    for (k, v) in map {
        match k {
            "foo" => assert_eq!(v, 42),
            "bar" => assert_eq!(v, 23),
            "baz" => assert_eq!(v, 142),
            "quox" => assert_eq!(v, 7),
            _ => unreachable!(),
        }
        items += 1;
    }
    assert_eq!(items, 4);
}

#[test]
fn test_entry() {
    let mut map = HashMap::new();
    map.insert("foo", 42);
    map.insert("bar", 23);
    map.insert("baz", 142);
    map.insert("quox", 7);
    let entry = map.entry("foo");
    assert_eq!(entry.or_insert(42), &42);
    let entry = map.entry("baz");
    assert_eq!(entry.or_insert(42), &142);
    let entry = map.entry("quox");
    assert_eq!(entry.or_insert(42), &7);
    let entry = map.entry("foobar");
    assert_eq!(entry.or_insert(42), &42);
    let entry = map.entry("foobar");
    assert_eq!(entry.or_insert_with(|| 42), &42);
    let entry = map.entry("foobar");
    assert_eq!(entry.or_default(), &42);
}

#[test]
fn test_from_iterator() {
    let map: HashMap<_, _> = [("foo", 42), ("bar", 23), ("baz", 142), ("quox", 7)]
        .iter()
        .cloned()
        .collect();
    assert_eq!(map.len(), 4);
    assert_eq!(map["foo"], 42);
    assert_eq!(map["bar"], 23);
    assert_eq!(map["baz"], 142);
    assert_eq!(map["quox"], 7);
}
