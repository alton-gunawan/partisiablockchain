use pbc_contract_common::sorted_vec_map::SortedVec;

fn setup_simple_set() -> SortedVec<u8> {
    let mut set: SortedVec<u8> = SortedVec::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set
}

#[test]
fn clear() {
    let mut set = SortedVec::new();
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
fn pop_last() {
    let mut set: SortedVec<u8> = SortedVec::new();
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
fn len() {
    let mut set = SortedVec::new();
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
    let mut set = SortedVec::new();
    assert!(set.is_empty());
    set.insert(1);
    assert!(!set.is_empty());
}

#[test]
fn iterator() {
    let mut set: SortedVec<u32> = SortedVec::new();
    set.insert(1);
    set.insert(2);

    assert_eq!(set.iter().sum::<u32>(), 3u32);
}
