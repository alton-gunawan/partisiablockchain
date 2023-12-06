use pbc_contract_common::avl_tree_map::AvlTreeMap;
use pbc_lib::wasm_avl::deserialize_avl_tree;
use pbc_traits::ReadWriteState;

#[test]
fn debug() {
    let tree: AvlTreeMap<i32, u128> = AvlTreeMap::new();

    assert_eq!(
        format!("{tree:?}"),
        "AvlTreeMap { key_type: PhantomData<i32>, value_type: PhantomData<u128>, tree_id: 0 }"
    );
}

#[test]
fn avl_tree_map() {
    let tree: AvlTreeMap<i32, u128> = AvlTreeMap::new();
    assert_eq!(tree.len(), 0);
    assert!(tree.is_empty());
    tree.insert(1, 12);
    assert_eq!(tree.len(), 1);
    assert!(!tree.is_empty());
    tree.insert(2, 42);
    assert_eq!(tree.len(), 2);
    assert!(!tree.is_empty());
    assert_eq!(tree.get(&1).unwrap(), 12);
    assert_eq!(tree.get(&2).unwrap(), 42);
    assert_eq!(tree.get(&3), None);

    tree.remove(&2);

    assert_eq!(tree.get(&2), None);

    assert!(!tree.contains_key(&2));
    assert!(tree.contains_key(&1));
}

#[test]
fn avl_tree_map_with_blobs() {
    let tree: AvlTreeMap<i32, Vec<u8>> = AvlTreeMap::new();
    assert_eq!(tree.len(), 0);
    assert!(tree.is_empty());
    tree.insert(1, vec![0, 1, 2, 3]);
    assert_eq!(tree.len(), 1);
    assert!(!tree.is_empty());
    tree.insert(2, vec![]);
    assert_eq!(tree.len(), 2);
    assert!(!tree.is_empty());
    tree.insert(99, vec![0xFF; 1000]);
    assert_eq!(tree.len(), 3);
    assert!(!tree.is_empty());
    assert_eq!(tree.get(&1).unwrap(), vec![0, 1, 2, 3]);
    assert_eq!(tree.get(&2).unwrap(), vec![]);
    assert_eq!(tree.get(&3), None);
    assert_eq!(tree.get(&98), None);
    assert_eq!(tree.get(&99).unwrap(), vec![0xFF; 1000]);
    assert_eq!(tree.get(&100), None);

    tree.remove(&2);

    assert_eq!(tree.get(&2), None);

    assert!(!tree.contains_key(&2));
    assert!(tree.contains_key(&1));
}

#[test]
fn avl_deserialize() {
    let bytes: Vec<u8> = vec![
        1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 2, 0, 0, 0, 1, 1, 1, 0, 0, 0, 4, 1, 1, 0, 0, 0, 1, 1, 1,
        1, 0, 0, 0, 8, 1, 1, 0, 0, 0, 0,
    ];
    deserialize_avl_tree(&mut bytes.as_slice());
    let tree: AvlTreeMap<u8, bool> = AvlTreeMap::state_read_from(&mut vec![0, 0, 0, 0].as_slice());

    assert!(tree.get(&4).unwrap());
    assert!(!tree.get(&8).unwrap());
    assert_eq!(tree.get(&1), None);
}

#[test]
fn avl_iter() {
    let tree: AvlTreeMap<i32, u128> = AvlTreeMap::new();
    tree.insert(3, 32);
    tree.insert(1, 12);
    tree.insert(4, 42);
    tree.insert(2, 22);
    tree.insert(256, 2562);
    tree.insert(257, 2572);
    let values: Vec<(i32, u128)> = tree.iter().collect();
    assert_eq!(
        values,
        [(256, 2562), (1, 12), (257, 2572), (2, 22), (3, 32), (4, 42)]
    );
}

#[test]
fn avl_iter_too_much() {
    let tree: AvlTreeMap<i32, i32> = AvlTreeMap::new();
    tree.insert(3, 32);
    let mut iter = tree.iter();
    assert_eq!(iter.next(), Some((3, 32)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn avl_iter_not_serializable_by_copy() {
    let tree: AvlTreeMap<i32, Option<u128>> = AvlTreeMap::new();
    tree.insert(3, Some(32));
    tree.insert(1, None);
    tree.insert(4, Some(42));
    tree.insert(2, Some(22));
    let values: Vec<(i32, Option<u128>)> = tree.iter().collect();
    assert_eq!(
        values,
        [(1, None), (2, Some(22)), (3, Some(32)), (4, Some(42))]
    );
}
