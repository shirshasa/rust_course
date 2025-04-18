#[cfg(any(
    feature = "test-basic",
    feature = "test-insert",
    feature = "test-removal",
    feature = "test-retain",
    feature = "test-iteration",
    feature = "test-range-queries",
    feature = "test-from-iterator",
    feature = "test-into-iterator",
    feature = "test-traits",
    feature = "test-no-extra-deps",
))]
use two_way_map::TwoWayMap;

#[cfg(any(
    feature = "test-basic",
    feature = "test-removal",
    feature = "test-into-iterator",
    feature = "test-no-extra-deps",
))]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct SomeStruct {
    a: i32,
    b: i32,
}

#[cfg(feature = "test-basic")]
#[test]
fn test_new() {
    let map = TwoWayMap::<i32, i32>::new();
    assert_eq!(map.len(), 0);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_new_no_copy_trait() {
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert_eq!(map.len(), 0);

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);

    assert_eq!(map.len(), 1);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_different_types_new() {
    let mut map = TwoWayMap::<SomeStruct, i32>::new();
    assert_eq!(map.len(), 0);

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, 5);
    map.insert(struct2, 6);

    assert_eq!(map.len(), 2);

    assert_eq!(map.get_by_left(&SomeStruct { a: 1, b: 2 }), Some(&5));
    assert_eq!(map.get_by_right(&5), Some(&SomeStruct { a: 1, b: 2 }));
}

#[cfg(feature = "test-basic")]
#[test]
fn test_is_empty() {
    let mut map = TwoWayMap::<i32, i32>::new();
    assert!(map.is_empty());

    map.insert(1, 2);
    assert!(!map.is_empty());

    map.insert(3, 4);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[cfg(feature = "test-basic")]
#[test]
fn test_is_empty_some_struct() {
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert!(map.is_empty());

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[cfg(feature = "test-basic")]
#[test]
fn test_is_empty_string() {
    let mut map = TwoWayMap::<String, String>::new();
    assert!(map.is_empty());

    let s1 = String::from("hello");
    let s2 = String::from("world");
    map.insert(s1.clone(), s2.clone());
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[cfg(feature = "test-basic")]
#[test]
fn test_clear() {
    let mut map = TwoWayMap::<SomeStruct, i32>::new();
    assert!(map.is_empty());
    map.insert(SomeStruct { a: 1, b: 2 }, 3);
    map.insert(SomeStruct { a: 4, b: 5 }, 6);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_clear_string() {
    let mut map = TwoWayMap::<String, String>::new();
    assert!(map.is_empty());
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.get_by_left(&String::from("hello")), None);
}

#[cfg(feature = "test-removal")]
#[test]
fn test_len() {
    let mut map = TwoWayMap::new();
    assert_eq!(map.len(), 0);
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
    map.insert(3, 4);
    assert_eq!(map.len(), 2);
    map.insert(1, 5);
    assert_eq!(map.len(), 2);

    map.remove_by_left(&1);
    assert_eq!(map.len(), 1);
}

#[cfg(feature = "test-removal")]
#[test]
fn test_len_string() {
    let mut map = TwoWayMap::<String, String>::new();
    assert_eq!(map.len(), 0);
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    map.insert(String::from("foo"), String::from("bar"));
    assert_eq!(map.len(), 2);
    map.insert(String::from("hello"), String::from("baz"));
    assert_eq!(map.len(), 2);

    map.remove_by_left(&String::from("hello"));
    assert_eq!(map.len(), 1);
}

#[cfg(feature = "test-insert")]
#[test]
fn test_insert() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&2), Some(&1));
}

#[cfg(feature = "test-basic")]
#[test]
fn test_insert_string() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get_by_left(&String::from("hello")),
        Some(&String::from("world"))
    );
    assert_eq!(
        map.get_by_right(&String::from("world")),
        Some(&String::from("hello"))
    );

    map.insert(String::from("foo"), String::from("bar"));
    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get_by_left(&String::from("foo")),
        Some(&String::from("bar"))
    );
    assert_eq!(
        map.get_by_right(&String::from("bar")),
        Some(&String::from("foo"))
    );
}

#[cfg(feature = "test-insert")]
#[test]
fn test_insert_no_overwrite() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);

    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(1, 3), Err((1, 3)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(3, 2), Err((3, 2)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(3, 4), Ok(()));
    assert_eq!(map.len(), 2);
}

#[cfg(feature = "test-insert")]
#[test]
fn test_insert_no_overwrite_string() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    assert_eq!(
        map.insert_no_overwrite(String::from("hello"), String::from("foo")),
        Err((String::from("hello"), String::from("foo")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(
        map.insert_no_overwrite(String::from("hi"), String::from("world")),
        Err((String::from("hi"), String::from("world")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(
        map.insert_no_overwrite(String::from("foo"), String::from("bar")),
        Ok(())
    );
    assert_eq!(map.len(), 2);
}

#[cfg(feature = "test-removal")]
#[test]
fn test_remove_by_left() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);

    assert_eq!(map.len(), 2);
    assert_eq!(map.remove_by_left(&1), Some((1, 2)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&1), None);
    assert_eq!(map.get_by_right(&2), None);

    assert_eq!(map.get_by_left(&3), Some(&4));
}

#[cfg(feature = "test-removal")]
#[test]
fn test_remove_by_left_string() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.remove_by_left(&String::from("hello")),
        Some((String::from("hello"), String::from("world")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&String::from("hello")), None);
    assert_eq!(map.get_by_right(&String::from("world")), None);

    assert_eq!(
        map.get_by_left(&String::from("foo")),
        Some(&String::from("bar"))
    );
}

#[cfg(feature = "test-removal")]
#[test]
fn test_remove_by_right() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);

    assert_eq!(map.len(), 2);
    assert_eq!(map.remove_by_right(&2), Some((2, 1)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&1), None);
    assert_eq!(map.get_by_right(&2), None);

    assert_eq!(map.get_by_left(&3), Some(&4));
}

#[cfg(feature = "test-removal")]
#[test]
fn test_remove_by_right_string() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.remove_by_right(&String::from("world")),
        Some((String::from("world"), String::from("hello")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&String::from("hello")), None);
    assert_eq!(map.get_by_right(&String::from("world")), None);

    assert_eq!(
        map.get_by_left(&String::from("foo")),
        Some(&String::from("bar"))
    );
}

#[cfg(feature = "test-removal")]
#[test]
fn test_remove_some_struct() {
    // by left
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert_eq!(map.len(), 0);

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);

    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get_by_left(&SomeStruct { a: 1, b: 2 }),
        Some(&SomeStruct { a: 3, b: 4 })
    );

    map.remove_by_left(&SomeStruct { a: 1, b: 2 });

    assert_eq!(map.len(), 0);
    assert_eq!(map.get_by_left(&SomeStruct { a: 1, b: 2 }), None);

    // by right
    let struct3 = SomeStruct { a: 5, b: 6 };
    let struct4 = SomeStruct { a: 7, b: 8 };
    map.insert(struct3, struct4);

    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get_by_left(&SomeStruct { a: 5, b: 6 }),
        Some(&SomeStruct { a: 7, b: 8 })
    );

    map.remove_by_right(&SomeStruct { a: 7, b: 8 });

    assert_eq!(map.len(), 0);
    assert_eq!(map.get_by_left(&SomeStruct { a: 5, b: 6 }), None);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_get() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&2), Some(&1));
    assert_eq!(map.get_by_left(&3), Some(&4));
    assert_eq!(map.get_by_right(&4), Some(&3));
    assert_eq!(map.get_by_left(&5), None);
    assert_eq!(map.get_by_right(&6), None);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_get_string() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(
        map.get_by_left(&String::from("hello")),
        Some(&String::from("world"))
    );
    assert_eq!(
        map.get_by_right(&String::from("world")),
        Some(&String::from("hello"))
    );
    assert_eq!(
        map.get_by_left(&String::from("foo")),
        Some(&String::from("bar"))
    );
    assert_eq!(
        map.get_by_right(&String::from("bar")),
        Some(&String::from("foo"))
    );
    assert_eq!(map.get_by_left(&String::from("baz")), None);
    assert_eq!(map.get_by_right(&String::from("qux")), None);
}

#[cfg(feature = "test-basic")]
#[test]
fn test_contains() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.contains_left(&1));
    assert!(map.contains_right(&2));
    assert!(map.contains_left(&3));
    assert!(map.contains_right(&4));

    assert!(!map.contains_left(&5));
    assert!(!map.contains_right(&6));

    let mut map3 = TwoWayMap::new();
    map3.insert("hello", "world");
    assert!(map3.contains_left(&"hello"));
    assert!(map3.contains_right(&"world"));
    assert!(!map3.contains_left(&"foo"));
}

#[cfg(feature = "test-basic")]
#[test]
fn test_contains_some_struct() {
    let mut map2 = TwoWayMap::new();
    let s1 = SomeStruct { a: 1, b: 2 };
    let s2 = SomeStruct { a: 3, b: 4 };

    map2.insert(s1, s2);
    assert!(!map2.contains_left(&SomeStruct { a: 5, b: 6 }));
    assert!(!map2.contains_right(&SomeStruct { a: 7, b: 8 }));
}

#[cfg(feature = "test-basic")]
#[test]
fn test_contains_string() {
    let mut map4 = TwoWayMap::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    map4.insert(s1.clone(), s2.clone());
    assert!(map4.contains_left(&s1));
    assert!(map4.contains_right(&s2));
    assert!(!map4.contains_left(&String::from("foo")));
    assert!(!map4.contains_right(&String::from("bar")));
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_pairs() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], (&1, &2));
    assert_eq!(pairs[1], (&3, &4));
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_pairs_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[1], (&String::from("hello"), &String::from("world")));
    assert_eq!(pairs[0], (&String::from("foo"), &String::from("bar")));
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_pairs_ascending() {
    // test that output is ascending
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(7, 8);
    map.insert(9, 10);
    map.insert(5, 6);

    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 5);
    assert_eq!(pairs[0], (&1, &2));
    assert_eq!(pairs[1], (&3, &4));
    assert_eq!(pairs[2], (&5, &6));
    assert_eq!(pairs[3], (&7, &8));
    assert_eq!(pairs[4], (&9, &10));
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_pairs_are_not_consumed() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], (&1, &2));
    assert_eq!(pairs[1], (&3, &4));

    // Check that the map is still intact
    assert_eq!(map.len(), 2);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&4), Some(&3));
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_pairs_are_not_consumed_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[1], (&String::from("hello"), &String::from("world")));
    assert_eq!(pairs[0], (&String::from("foo"), &String::from("bar")));

    // Check that the map is still intact
    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get_by_left(&String::from("hello")),
        Some(&String::from("world"))
    );
    assert_eq!(
        map.get_by_right(&String::from("bar")),
        Some(&String::from("foo"))
    );
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_left_values() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 20);
    map.insert(100, 0);
    map.insert(3, 4);

    let left_values: Vec<_> = map.left_values().collect();
    assert_eq!(left_values.len(), 4);
    assert_eq!(left_values, vec![&1, &3, &5, &100]);
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_left_values_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let left_values: Vec<_> = map.left_values().collect();
    assert_eq!(left_values.len(), 3);
    assert_eq!(
        left_values,
        vec![
            &String::from("baz"),
            &String::from("foo"),
            &String::from("hello"),
        ]
    );
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_right_values() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 20);
    map.insert(100, 0);
    map.insert(3, 4);
    let right_values: Vec<_> = map.right_values().collect();
    assert_eq!(right_values.len(), 4);
    assert_eq!(right_values, vec![&0, &2, &4, &20]);
}

#[cfg(feature = "test-iteration")]
#[test]
fn test_right_values_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let right_values: Vec<_> = map.right_values().collect();
    assert_eq!(right_values.len(), 3);
    assert_eq!(
        right_values,
        vec![
            &String::from("bar"),
            &String::from("qux"),
            &String::from("world")
        ]
    );
}

#[cfg(feature = "test-range-queries")]
#[test]
fn test_range() {
    use std::ops::Bound::Included;

    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);
    let range: Vec<_> = map.left_range(1..=3).collect();

    assert_eq!(range.len(), 2);
    assert_eq!(range, vec![(&1, &2), (&3, &4)]);

    let range: Vec<_> = map.right_range(2..=4).collect();

    assert_eq!(range.len(), 2);
    assert_eq!(range, vec![(&2, &1), (&4, &3)]);

    let range: Vec<_> = map.left_range((Included(&4), Included(&8))).collect();
    assert_eq!(range.len(), 1);
    assert_eq!(range, vec![(&5, &6)]);
}

#[cfg(feature = "test-range-queries")]
#[test]
fn test_range_literal_string() {
    let mut map = TwoWayMap::new();
    map.insert("hello", "world");
    map.insert("foo", "bar");
    map.insert("baz", "qux");

    let range: Vec<_> = map.left_range("a"..="foo").collect();
    assert_eq!(range.len(), 2);
    assert_eq!(range, vec![(&"baz", &"qux"), (&"foo", &"bar")]);

    let range: Vec<_> = map.left_range("a".."f").collect();
    assert_eq!(range.len(), 1);
    assert_eq!(range, vec![(&"baz", &"qux")]);
}

#[cfg(feature = "test-range-queries")]
#[test]
fn test_range_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let start = "a".to_string();
    let end = "foo".to_string();
    let range: Vec<_> = map.left_range(&start..=&end).collect();
    assert_eq!(range.len(), 2);
    assert_eq!(
        range,
        vec![
            (&String::from("baz"), &String::from("qux")),
            (&String::from("foo"), &String::from("bar"))
        ]
    );

    let range: Vec<_> = map.left_range(&start..&end).collect();
    assert_eq!(range.len(), 1);
    assert_eq!(range, vec![(&String::from("baz"), &String::from("qux"))]);
}

#[cfg(feature = "test-range-queries")]
#[test]
fn test_left_range_string_one_more() {
    let mut map = TwoWayMap::new();
    map.insert("Alice".to_string(), "Engineer".to_string());
    map.insert("Bob".to_string(), "Doctor".to_string());
    map.insert("Charlie".to_string(), "Teacher".to_string());
    map.insert("Dave".to_string(), "Artist".to_string());

    let range = "Bob".to_string().."Dave".to_string();
    let result: Vec<(&String, &String)> = map.left_range(range).collect();
    assert_eq!(
        result,
        vec![
            (&"Bob".to_string(), &"Doctor".to_string()),
            (&"Charlie".to_string(), &"Teacher".to_string())
        ]
    );
}

#[cfg(feature = "test-retain")]
#[test]
fn test_retain() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(2, 4);
    map.insert(5, 6);
    map.insert(7, 8);

    assert_eq!(map.len(), 4);

    map.retain(|left, right| left % 2 == 0 && right % 2 == 0);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&2), Some(&4));
    assert_eq!(map.get_by_right(&4), Some(&2));
}

#[cfg(feature = "test-retain")]
#[test]
fn test_retain_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));
    map.insert(String::from("quux"), String::from("corge"));

    assert_eq!(map.len(), 4);

    map.retain(|left, right| left == "foo" || right == "qux");

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get_by_left(&String::from("foo")),
        Some(&String::from("bar"))
    );
    assert_eq!(
        map.get_by_right(&String::from("qux")),
        Some(&String::from("baz"))
    );
}

#[cfg(feature = "test-traits")]
#[test]
fn test_clone() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    let map2 = map.clone();

    assert_eq!(map.len(), map2.len());
    assert_eq!(
        map.pairs().collect::<Vec<_>>(),
        map2.pairs().collect::<Vec<_>>()
    );
}

#[cfg(feature = "test-traits")]
#[test]
fn test_clone_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let map2 = map.clone();

    assert_eq!(map.len(), map2.len());
    assert_eq!(
        map.pairs().collect::<Vec<_>>(),
        map2.pairs().collect::<Vec<_>>()
    );
}

#[cfg(feature = "test-traits")]
#[test]
fn test_extend() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);

    let new_pairs = vec![(5, 6), (7, 8)];
    map.extend(new_pairs);

    assert_eq!(map.len(), 4);
    assert_eq!(map.get_by_left(&5), Some(&6));
    assert_eq!(map.get_by_right(&8), Some(&7));
}

#[cfg(feature = "test-traits")]
#[test]
fn test_extend_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    let new_pairs = vec![
        (String::from("baz"), String::from("qux")),
        (String::from("quux"), String::from("corge")),
    ];
    map.extend(new_pairs);

    assert_eq!(map.len(), 4);
    assert_eq!(
        map.get_by_left(&String::from("baz")),
        Some(&String::from("qux"))
    );
    assert_eq!(
        map.get_by_right(&String::from("corge")),
        Some(&String::from("quux"))
    );
}

#[cfg(feature = "test-from-iterator")]
#[test]
fn test_from_iterator() {
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    let map: TwoWayMap<i32, i32> = TwoWayMap::from_iter(pairs);

    assert_eq!(map.len(), 3);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&4), Some(&3));
    assert_eq!(map.get_by_left(&5), Some(&6));
}

#[cfg(feature = "test-from-iterator")]
#[test]
fn test_from_iterator_string() {
    let pairs = vec![
        (String::from("hello"), String::from("world")),
        (String::from("foo"), String::from("bar")),
    ];
    let map: TwoWayMap<String, String> = TwoWayMap::from_iter(pairs);

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get_by_left(&String::from("hello")),
        Some(&String::from("world"))
    );
    assert_eq!(
        map.get_by_right(&String::from("bar")),
        Some(&String::from("foo"))
    );
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 6);
    map.insert(3, 4);

    let mut iter = map.into_iter();
    assert_eq!(iter.next(), Some((1, 2)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), Some((5, 6)));

    assert_eq!(iter.next(), None);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_string() {
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let mut iter = map.into_iter();
    assert_eq!(
        iter.next(),
        Some((String::from("baz"), String::from("qux")))
    );
    assert_eq!(
        iter.next(),
        Some((String::from("foo"), String::from("bar")))
    );
    assert_eq!(
        iter.next(),
        Some((String::from("hello"), String::from("world")))
    );

    assert_eq!(iter.next(), None);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_with_clone() {
    let mut map1 = TwoWayMap::new();
    map1.insert(1, 2);
    map1.insert(5, 6);
    map1.insert(3, 4);

    let map = map1.clone();

    let mut iter = map.into_iter();
    assert_eq!(iter.next(), Some((1, 2)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), Some((5, 6)));

    assert_eq!(iter.next(), None);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_string_with_clone() {
    let mut map1 = TwoWayMap::new();
    map1.insert(String::from("hello"), String::from("world"));
    map1.insert(String::from("foo"), String::from("bar"));
    map1.insert(String::from("baz"), String::from("qux"));

    let map = map1.clone();

    let mut iter = map.into_iter();
    assert_eq!(
        iter.next(),
        Some((String::from("baz"), String::from("qux")))
    );
    assert_eq!(
        iter.next(),
        Some((String::from("foo"), String::from("bar")))
    );
    assert_eq!(
        iter.next(),
        Some((String::from("hello"), String::from("world")))
    );

    assert_eq!(iter.next(), None);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_in_for_loop() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    for x in map.into_iter() {
        println!("x: {:?}", x);
    }

    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    for x in map {
        println!("x: {:?}", x);
    }

    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 6);
    map.insert(3, 4);
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_into_iter_no_clone_trait() {
    let mut map = TwoWayMap::new();
    map.insert(SomeStruct { a: 101, b: 202 }, SomeStruct { a: 303, b: 403 });
    map.insert(SomeStruct { a: 0, b: 1 }, SomeStruct { a: 1, b: 0 });

    for x in map {
        println!("x: {:?}", x);
    }
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_ref_into_iter() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    for x in &map {
        println!("x: {:?}", x);
    }
}

#[cfg(feature = "test-into-iterator")]
#[test]
fn test_ref_into_iter_no_clone_trait() {
    let mut map = TwoWayMap::new();
    map.insert(SomeStruct { a: 101, b: 202 }, SomeStruct { a: 303, b: 403 });
    map.insert(SomeStruct { a: 0, b: 1 }, SomeStruct { a: 1, b: 0 });

    for x in &map {
        println!("x: {:?}", x);
    }
}

#[cfg(feature = "test-traits")]
#[test]
fn test_debug() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    println!("{:?}", map);

    let mut map2 = TwoWayMap::new();
    map2.insert(String::from("hello"), String::from("world"));
    map2.insert(String::from("foo"), String::from("bar"));
    map2.insert(String::from("baz"), String::from("qux"));

    println!("{:?}", map2);
}

#[cfg(feature = "test-traits")]
#[test]
fn test_default() {
    let map: TwoWayMap<i32, i32> = TwoWayMap::default();
    assert_eq!(map.len(), 0);

    let map2: TwoWayMap<String, String> = TwoWayMap::default();
    assert_eq!(map2.len(), 0);
}

#[cfg(feature = "test-no-extra-deps")]
#[test]
fn test_default_no_default_trait() {
    let map: TwoWayMap<SomeStruct, SomeStruct> = TwoWayMap::default();
    assert_eq!(map.len(), 0);
}
