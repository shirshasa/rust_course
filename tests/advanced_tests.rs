use two_way_map::TwoWayMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Container<T>(T);

impl<T> std::borrow::Borrow<T> for Container<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

#[test]
pub(crate) fn test_get_str() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.get_by_left("hello"), Some(&String::from("world")));
    assert_eq!(map.get_by_right("world"), Some(&String::from("hello")));
    assert_eq!(map.get_by_left("foo"), Some(&String::from("bar")));
    assert_eq!(map.get_by_right("bar"), Some(&String::from("foo")));
    assert_eq!(map.get_by_left("baz"), None);
    assert_eq!(map.get_by_right("qux"), None);
}

#[test]
pub(crate) fn test_get_with_custom_borrow() {
    let mut map = TwoWayMap::new();
    map.insert(Container(0), String::from("world"));
    map.insert(Container(1), String::from("bar"));

    assert_eq!(map.get_by_left(&0), Some(&String::from("world")));
    assert_eq!(map.get_by_right("world"), Some(&Container(0)));
    assert_eq!(map.get_by_left(&1), Some(&String::from("bar")));
    assert_eq!(map.get_by_right("bar"), Some(&Container(1)));
    assert_eq!(map.get_by_left(&2), None);
    assert_eq!(map.get_by_right("qux"), None);
}

#[test]
pub(crate) fn test_contains_str() {
    let mut map = TwoWayMap::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    map.insert(s1, s2);
    assert!(map.contains_left("hello"));
    assert!(map.contains_right("world"));
    assert!(!map.contains_left("foo"));
    assert!(!map.contains_right("bar"));
}

#[test]
pub(crate) fn test_remove_by_left_str() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.remove_by_left("hello"),
        Some((String::from("hello"), String::from("world")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left("hello"), None);
    assert_eq!(map.get_by_right("world"), None);

    assert_eq!(map.get_by_left("foo"), Some(&String::from("bar")));
}

#[test]
pub(crate) fn test_remove_by_right_str() {
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(
        map.remove_by_right("world"),
        Some((String::from("world"), String::from("hello")))
    );
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left("hello"), None);
    assert_eq!(map.get_by_right("world"), None);

    assert_eq!(map.get_by_left("foo"), Some(&String::from("bar")));
}
