mod recorder;

#[cfg(feature = "test-traits")]
#[test]
fn test_clone_counts() {
    use two_way_map::TwoWayMap;

    let (recorder, dummy) = recorder::create_dummy();

    let mut map = TwoWayMap::new();
    map.insert(1, dummy);
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);

    let map2 = map.clone();
    assert_eq!(recorder.borrow().clones, 1);
    assert_eq!(recorder.borrow().dropped, false);

    let _map3 = map2.clone();
    assert_eq!(recorder.borrow().clones, 2);
    assert_eq!(recorder.borrow().dropped, false);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_moves() {
    use two_way_map::TwoWayMap;

    let (recorder, dummy) = recorder::create_dummy();

    let mut map = TwoWayMap::new();
    map.insert(1, dummy);
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);

    let _map2 = map;
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_clone_counts() {
    use two_way_map::TwoWayMap;

    let (recorder, dummy) = recorder::create_dummy();

    {
        let mut map = TwoWayMap::new();
        map.insert(1, dummy);
        assert_eq!(recorder.borrow().clones, 0);
        assert_eq!(recorder.borrow().dropped, false);

        let mut iter = map.into_iter();
        assert_eq!(recorder.borrow().clones, 0);
        assert_eq!(recorder.borrow().dropped, false);

        let (key, value) = iter.next().unwrap();
        println!("key: {}, value: {:?}", key, value);
    }
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, true);
}

#[cfg(feature = "test-no-extra-deps")]
#[test]
fn test_non_ord() {
    #[derive(Debug)]
    struct NonOrd {}

    use two_way_map::TwoWayMap;

    let mut map: TwoWayMap<NonOrd, NonOrd> = TwoWayMap::new();
    assert_eq!(map.len(), 0);
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    assert_eq!(map.pairs().count(), 0);
    assert_eq!(map.left_values().count(), 0);
    assert_eq!(map.right_values().count(), 0);

    assert_eq!((&map).into_iter().count(), 0);
    assert_eq!(map.into_iter().count(), 0);
}
