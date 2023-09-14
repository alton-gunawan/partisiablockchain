use pbc_contract_common::sorted_vec_map::SortedVecSet;

fn setup_simple_set() -> SortedVecSet<u8> {
    let mut set: SortedVecSet<u8> = SortedVecSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set
}

#[test]
fn clear() {
    let mut set = SortedVecSet::new();
    set.insert(1);
    set.insert(2);
    set.clear();
    assert!(set.get(&1).is_none());
    assert!(set.get(&2).is_none());
    assert_eq!(set.len(), 0);
}

#[test]
fn get() {
    let set = setup_simple_set();
    assert_eq!(set.get(&1), Some(&1));
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&3), Some(&3));
    assert_eq!(set.get(&4), None);
    assert_eq!(set.get(&0), None);
}

#[test]
fn pop_first() {
    let mut set: SortedVecSet<u8> = SortedVecSet::new();
    assert!(set.pop_first().is_none());
    let mut set = setup_simple_set();
    assert_eq!(set.len(), 3);
    assert_eq!(set.pop_first(), Some(1));
    assert_eq!(set.len(), 2);
}

#[test]
fn pop_last() {
    let mut set: SortedVecSet<u8> = SortedVecSet::new();
    assert!(set.pop_last().is_none());
    let mut set = setup_simple_set();
    assert_eq!(set.len(), 3);
    assert_eq!(set.pop_last(), Some(3));
    assert_eq!(set.len(), 2);
}

#[test]
fn contains() {
    let set = setup_simple_set();
    assert!(set.contains(&1));
    assert!(!set.contains(&4));
}

#[test]
fn remove() {
    let mut set = setup_simple_set();
    assert!(set.remove(&4).is_none());
    assert_eq!(set.len(), 3);
    let removed = set.remove(&1);
    assert_eq!(set.len(), 2);
    assert_eq!(removed, Some(1));
}

#[test]
fn retain() {
    let mut set = SortedVecSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(10);
    set.insert(100);
    set.insert(3);
    set.insert(4);
    set.retain(|v| v < &mut 10);
    assert_eq!(set.len(), 4);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&10));
    assert!(!set.contains(&100));
    assert!(set.contains(&3));
    assert!(set.contains(&4));
}

#[test]
fn append() {
    let mut set = SortedVecSet::new();
    set.insert(1);
    set.insert(2);
    let mut other = SortedVecSet::new();
    other.insert(1);
    other.insert(3);
    set.append(&mut other);
    assert_eq!(set.len(), 3);
}

#[test]
fn range() {
    let mut set = SortedVecSet::new();
    set.insert(1);
    set.insert(4);
    set.insert(5);
    set.insert(10);
    let iter = set.range(1i32..5);
    let elements: Vec<&i32> = iter.collect();
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], &1);
    assert_eq!(elements[1], &4);
}

#[test]
fn into_iter() {
    let set = setup_simple_set();
    let elements: Vec<u8> = set.into_iter().collect();
    assert_eq!(elements.len(), 3);
    assert_eq!(elements, vec![1, 2, 3]);
}

#[test]
fn iter() {
    let set = setup_simple_set();
    let mut iter = set.iter();
    let first = iter.next().unwrap();
    assert_eq!(first, &1);
    let second = iter.next().unwrap();
    assert_eq!(second, &2);
    let third = iter.next().unwrap();
    assert_eq!(third, &3);
    assert!(iter.next().is_none());
}

#[test]
fn iter_2() {
    let set = setup_simple_set();
    let elements: Vec<&u8> = set.iter().collect();
    assert_eq!(elements, [&1, &2, &3]);
}

#[test]
fn into_iter_2() {
    let set = setup_simple_set();
    let elements: Vec<u8> = set.into_iter().collect();
    assert_eq!(elements, [1, 2, 3]);
}

#[test]
fn len() {
    let mut set = SortedVecSet::new();
    assert_eq!(set.len(), 0);
    set.insert(1);
    assert_eq!(set.len(), 1);
    set.insert(2);
    assert_eq!(set.len(), 2);
    set.remove(&3);
    assert_eq!(set.len(), 2);
    set.remove(&2);
    assert_eq!(set.len(), 1);
}

#[test]
fn is_empty() {
    let mut set = SortedVecSet::new();
    assert!(set.is_empty());
    set.insert(1);
    assert!(!set.is_empty());
}

#[test]
fn from_tuples() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 3];
    let set = SortedVecSet::from(arr);

    let mut other = SortedVecSet::new();
    other.insert(1);
    other.insert(2);
    other.insert(3);
    other.insert(4);
    other.insert(5);
    other.insert(6);
    other.insert(7);

    assert_eq!(set.len(), 7);
    assert_eq!(set, other);
}

#[test]
fn collect_from_vector() {
    let vec = vec![1, 2, 3, 4, 5, 3, 111, 6, 2, 1, 6, 5, 3, 6, 7, 3];

    let set: SortedVecSet<i32> = vec.into_iter().collect();
    assert_eq!(set.len(), 8);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(set.contains(&4));
    assert!(set.contains(&5));
    assert!(set.contains(&6));
    assert!(set.contains(&7));
    assert!(set.contains(&111));
}
