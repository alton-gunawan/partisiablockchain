use pbc_contract_common::sorted_vec_map::SortedVecMap;

fn setup_simple_map() -> SortedVecMap<u8, String> {
    let mut map: SortedVecMap<u8, String> = SortedVecMap::new();
    map.insert(1, "hello".to_string());
    map.insert(2, "world".to_string());
    map.insert(3, "123".to_string());
    map
}

#[test]
fn clear() {
    let mut map = SortedVecMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.clear();
    assert!(map.get(&1).is_none());
    assert!(map.get(&2).is_none());
    assert_eq!(map.len(), 0);
}

#[test]
fn get() {
    let map = setup_simple_map();
    assert_eq!(map.get(&1), Some(&"hello".to_string()));
    assert_eq!(map.get(&2), Some(&"world".to_string()));
    assert_eq!(map.get(&3), Some(&"123".to_string()));
    assert_eq!(map.get(&4), None);
    assert_eq!(map.get(&0), None);
}

#[test]
fn get_key_value() {
    let map = setup_simple_map();
    assert_eq!(map.get_key_value(&1), Some((&1, &"hello".to_string())));
    assert_eq!(map.get_key_value(&2), Some((&2, &"world".to_string())));
    assert_eq!(map.get_key_value(&3), Some((&3, &"123".to_string())));
    assert_eq!(map.get_key_value(&4), None);
    assert_eq!(map.get_key_value(&0), None);
}

#[test]
fn first_key_value() {
    let map: SortedVecMap<u8, String> = SortedVecMap::new();
    assert!(map.first_key_value().is_none());
    let map = setup_simple_map();
    assert_eq!(map.first_key_value(), Some((&1, &"hello".to_string())));
    assert_eq!(map.len(), 3);
}

#[test]
fn pop_first() {
    let mut map: SortedVecMap<u8, String> = SortedVecMap::new();
    assert!(map.pop_first().is_none());
    let mut map = setup_simple_map();
    assert_eq!(map.len(), 3);
    assert_eq!(map.pop_first(), Some((1, "hello".to_string())));
    assert_eq!(map.len(), 2);
}

#[test]
fn last_key_value() {
    let map: SortedVecMap<u8, String> = SortedVecMap::new();
    assert!(map.last_key_value().is_none());
    let map = setup_simple_map();
    assert_eq!(map.last_key_value(), Some((&3, &"123".to_string())));
    assert_eq!(map.len(), 3);
}

#[test]
fn pop_last() {
    let mut map: SortedVecMap<u8, String> = SortedVecMap::new();
    assert!(map.pop_last().is_none());
    let mut map = setup_simple_map();
    assert_eq!(map.len(), 3);
    assert_eq!(map.pop_last(), Some((3, "123".to_string())));
    assert_eq!(map.len(), 2);
}

#[test]
fn contains_key() {
    let map = setup_simple_map();
    assert!(map.contains_key(&1));
    assert!(!map.contains_key(&4));
}

#[test]
fn get_mut() {
    let mut map = setup_simple_map();
    assert!(map.get_mut(&4).is_none());
    let reference = map.get_mut(&3);
    assert!(reference.is_some());
    let reference = reference.unwrap();
    *reference = "awesome".to_string();

    assert_eq!(map[&3], "awesome".to_string());
}

#[test]
fn remove() {
    let mut map = setup_simple_map();
    assert!(map.remove(&4).is_none());
    assert_eq!(map.len(), 3);
    let removed = map.remove(&1);
    assert_eq!(map.len(), 2);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap(), "hello".to_string());
}

#[test]
fn remove_entry() {
    let mut map = setup_simple_map();
    assert!(map.remove_entry(&4).is_none());
    assert_eq!(map.len(), 3);
    let removed = map.remove_entry(&1);
    assert_eq!(map.len(), 2);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap(), (1, "hello".to_string()));
}

#[test]
fn retain() {
    let mut map = SortedVecMap::new();
    map.insert(1, 5);
    map.insert(2, 20);
    map.insert(3, 10);
    map.insert(4, 40);
    map.retain(|k, v| v <= &mut 10 && k > &1);
    assert_eq!(map.len(), 1);
    assert_eq!(map[&3], 10);
    assert!(map.get(&1).is_none());
    assert!(map.get(&2).is_none());
    assert!(map.get(&4).is_none());
}

#[test]
fn append() {
    let mut map = SortedVecMap::new();
    map.insert(1, "Should be replaced".to_string());
    map.insert(2, "Oh hi".to_string());
    let mut other = SortedVecMap::new();
    other.insert(1, "I replaced you".to_string());
    other.insert(3, "new element".to_string());
    map.append(&mut other);
    assert_eq!(map.len(), 3);
    assert_eq!(map[&1], "I replaced you".to_string());
    assert_eq!(map[&2], "Oh hi".to_string());
    assert_eq!(map[&3], "new element".to_string());
}

#[test]
fn range() {
    let mut map = SortedVecMap::new();
    map.insert(1, 1);
    map.insert(4, 2);
    map.insert(5, 3);
    map.insert(10, 4);
    let iter = map.range(1..5);
    let elements: Vec<(&i32, &i32)> = iter.collect();
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], (&1, &1));
    assert_eq!(elements[1], (&4, &2));
}

#[test]
fn range_mut() {
    let mut map = SortedVecMap::new();
    map.insert(1, 1);
    map.insert(4, 2);
    map.insert(5, 3);
    map.insert(10, 4);
    let iter = map.range_mut(1..5);
    let mut elements: Vec<(&i32, &mut i32)> = iter.collect();
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], (&1, &mut 1));
    assert_eq!(elements[1], (&4, &mut 2));

    let entry = &mut elements[0];
    *entry.1 = 25;
    assert_eq!(elements[0], (&1, &mut 25));
    assert_eq!(map[&1], 25);
}

#[test]
fn into_keys() {
    let map = setup_simple_map();
    let keys: Vec<u8> = map.into_keys().collect();
    assert_eq!(keys.len(), 3);
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn into_values() {
    let map = setup_simple_map();
    let keys: Vec<String> = map.into_values().collect();
    assert_eq!(keys.len(), 3);
    assert_eq!(
        keys,
        vec!["hello".to_string(), "world".to_string(), "123".to_string()]
    );
}

#[test]
fn iter() {
    let map = setup_simple_map();
    let mut iter = map.iter();
    let (first_key, first_value) = iter.next().unwrap();
    assert_eq!(first_key, &1);
    assert_eq!(first_value, &"hello".to_string());
    let (second_key, second_value) = iter.next().unwrap();
    assert_eq!(second_key, &2);
    assert_eq!(second_value, &"world".to_string());
    let (third_key, third_value) = iter.next().unwrap();
    assert_eq!(third_key, &3);
    assert_eq!(third_value, &"123".to_string());
    assert!(iter.next().is_none());
}

#[test]
fn iter_mut() {
    let mut map = SortedVecMap::from([(1, 1), (2, 2), (3, 3)]);

    // add 10 to the value if the key isn't 1
    for (key, value) in map.iter_mut() {
        if key != &1 {
            *value += 10;
        }
    }
    assert_eq!(map[&1], 1);
    assert_eq!(map[&2], 12);
    assert_eq!(map[&3], 13);
}

#[test]
fn keys() {
    let map = setup_simple_map();
    let keys: Vec<&u8> = map.keys().collect();
    assert_eq!(keys, [&1, &2, &3]);
}

#[test]
fn values() {
    let map = setup_simple_map();
    let values: Vec<&String> = map.values().collect();
    assert_eq!(
        values,
        [
            &"hello".to_string(),
            &"world".to_string(),
            &"123".to_string()
        ]
    );
}

#[test]
fn values_mut() {
    let mut map = setup_simple_map();
    for value in map.values_mut() {
        value.push('!');
    }

    let values: Vec<String> = map.values().cloned().collect();
    assert_eq!(
        values,
        [
            "hello!".to_string(),
            "world!".to_string(),
            "123!".to_string()
        ]
    );
}

#[test]
fn len() {
    let mut map = SortedVecMap::new();
    assert_eq!(map.len(), 0);
    map.insert(1, 1);
    assert_eq!(map.len(), 1);
    map.insert(2, 2);
    assert_eq!(map.len(), 2);
    map.remove(&3);
    assert_eq!(map.len(), 2);
    map.remove(&2);
    assert_eq!(map.len(), 1);
}

#[test]
fn is_empty() {
    let mut map = SortedVecMap::new();
    assert!(map.is_empty());
    map.insert(1, 1);
    assert!(!map.is_empty());
}

#[test]
fn from_tuples() {
    let map = SortedVecMap::from([(1, 1), (2, 20), (3, 3), (4, 40), (5, 5), (6, 60), (7, 7)]);

    let mut other = SortedVecMap::new();
    other.insert(1, 1);
    other.insert(2, 20);
    other.insert(3, 3);
    other.insert(4, 40);
    other.insert(5, 5);
    other.insert(6, 60);
    other.insert(7, 7);

    assert_eq!(map, other);
}
