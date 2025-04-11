use std::collections;
use std::{fmt::Debug, ops::RangeBounds, ops::Bound::Included, rc::Rc};



#[derive(Debug, Default)]
struct TwoWayMap<L, R> {
    left_to_right: std::collections::BTreeMap<Rc<L>, Rc<R>>,
    right_to_left: std::collections::BTreeMap<Rc<R>, Rc<L>>,
}

impl<L: Ord, R: Ord> TwoWayMap<L, R> {
    pub fn new() -> Self {
        Self {
            left_to_right: std::collections::BTreeMap::new(),
            right_to_left: std::collections::BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.left_to_right.len()
    }

    pub fn is_empty(&self) -> bool {
        self.left_to_right.is_empty()
    }

    pub fn clear(&mut self) {
        self.left_to_right.clear();
        self.right_to_left.clear();
    }

    pub fn insert(&mut self, left: L, right: R) {
        // Check if left already exists
        if let Some(existing_right) = self.left_to_right.get(&left) {
            self.right_to_left.remove(existing_right);
        }
        // Check if right already exists
        if let Some(existing_left) = self.right_to_left.get(&right) {
            self.left_to_right.remove(existing_left);
        }

        let left = Rc::new(left);
        let right = Rc::new(right);

        self.left_to_right.insert(left.clone(), right.clone());
        self.right_to_left.insert(right, left);
    }

    pub fn insert_no_overwrite(&mut self, left: L, _right: R) -> Result<(), (L, R)> {
        //Check if left or right already exists
        if self.left_to_right.contains_key(&left) || self.right_to_left.contains_key(&_right) {
            return Err((left, _right));
        }
        self.insert(left, _right);
        Ok(())
    }

    pub fn remove_by_left(&mut self, left: &L) -> Option<(L, R)> {
        if let Some(right) = self.left_to_right.remove(left) {
            let left = self.right_to_left.remove(&right).unwrap();

            // self.left_to_right.remove(&left);

            // Convert Rc to L and R
            let left = Rc::try_unwrap(left).ok().unwrap();
            let right = Rc::try_unwrap(right).ok().unwrap();

            let pair = (left, right);

            return Some(pair);
        }
        None
    }

    pub fn remove_by_right(&mut self, right: &R) -> Option<(R, L)> {
        if let Some(left) = self.right_to_left.remove(right) {
            let right = self.left_to_right.remove(&left).unwrap().clone();
            // Convert Rc to L and R
            let left = Rc::try_unwrap(left).ok().unwrap();
            let right = Rc::try_unwrap(right).ok().unwrap();

            let pair = (right, left);

            return Some(pair);
        }
        None
    }

    pub fn get_by_left(&self, left: &L) -> Option<&R> {
        if let Some(right) = self.left_to_right.get(left) {
            // Should we increment rc???
            return Some(right.as_ref());
        }
        None
    }

    pub fn get_by_right(&self, right: &R) -> Option<&L> {
        if let Some(left) = self.right_to_left.get(right) {
            return Some(left.as_ref());
        }
        None
    }

    pub fn contains_left(&self, left: &L) -> bool {
        self.left_to_right.contains_key(left)
    }

    pub fn contains_right(&self, right: &R) -> bool {
        self.right_to_left.contains_key(right)
    }

    pub fn pairs(&self) -> impl Iterator<Item = (&L, &R)> {
        self.left_to_right.iter().map(|(left, right)| {
            let left = left.as_ref();
            let right = right.as_ref();
            (left, right)
        })
    }

    pub fn left_values(&self) -> impl Iterator<Item = &L> {
        self.left_to_right.keys().map(|k| k.as_ref())
    }

    pub fn right_values(&self) -> impl Iterator<Item = &R> {
        self.right_to_left.keys().map(|k| k.as_ref())
    }

    pub fn left_range<T>(&self, range: T) -> impl Iterator<Item = (&L, &R)>
    where
        L: Ord,
        T: RangeBounds<L>,
    {
        self.left_to_right
            .range(range)
            .map(|(left, right)| (left.as_ref(), right.as_ref()))
    }

    pub fn right_range<T>(&self, range: T) -> impl Iterator<Item = (&R, &L)>
    where
        R: Ord,
        T: RangeBounds<R>,
    {
        self.right_to_left
            .range(range)
            .map(|(right, left)| (right.as_ref(), left.as_ref()))
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&L, &R) -> bool,
    {
        self.left_to_right
            .retain(|left, right| f(left.as_ref(), right.as_ref()));

        self.right_to_left
            .retain(|right, left| f(left.as_ref(), right.as_ref()));
    }


}

impl<L: Ord, R: Ord> Clone for TwoWayMap<L, R> 
where L: Clone, R: Clone{
    fn clone(&self) -> Self {
        let mut other = TwoWayMap::new();
        for (left, right) in self.left_to_right.iter() {
            let left =left.as_ref().clone();
            let right = right.as_ref().clone();
            other.insert(left, right);
        }
        other
    }
}

impl<L: Ord, R: Ord> Extend<(L, R)> for TwoWayMap<L, R> {
    fn extend<T: IntoIterator<Item = (L, R)>>(&mut self, iter: T) {
        for (left, right) in iter {
            self.insert(left, right);
        }
    }
}

impl<L: Ord, R: Ord> FromIterator<(L, R)> for TwoWayMap<L, R> {
    fn from_iter<T: IntoIterator<Item = (L, R)>>(iter: T) -> Self {
        let mut map = TwoWayMap::new();
        for (left, right) in iter {
            map.insert(left, right);
        }
        map
    }
}

struct IntoIter<L,R>{
    left_to_right_iter: std::collections::btree_map::IntoIter<Rc<L>, Rc<R>>,
    right_to_left: std::collections::BTreeMap<Rc<R>, Rc<L>>,
}


impl <L: Ord, R: Ord> IntoIter<L, R> {
    fn new(map: TwoWayMap<L, R>) -> Self {
        Self {
            left_to_right_iter: map.left_to_right.into_iter(),
            right_to_left: map.right_to_left,
        }
    }
    
} 

impl<L: Ord, R: Ord> Iterator for IntoIter<L, R> {
    type Item = (L, R);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left, right)) = self.left_to_right_iter.next() {
            // Remove the corresponding entry from right_to_left
            self.right_to_left.remove(&right);

            let left = Rc::try_unwrap(left).ok().unwrap();
            let right = Rc::try_unwrap(right).ok().unwrap();

            return Some((left, right));
        }
        None
    }
}

impl <L: Ord, R: Ord> IntoIterator for TwoWayMap<L, R> {
    type Item = (L, R);

    type IntoIter = IntoIter<L, R>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}


struct RefIter<'l, L, R>{
    iter: collections::btree_map::Iter<'l, Rc<L>, Rc<R>>,
}

impl <'l, L: Ord, R: Ord> RefIter<'l, L, R> {
    fn new(map_ref: &'l TwoWayMap<L, R>) -> Self {
        Self {iter:map_ref.left_to_right.iter()}
    }
}

impl<'l, L: Ord, R: Ord> Iterator for RefIter<'l, L, R> {
    type Item = (&'l L, &'l R);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left, right)) = self.iter.next() {
            let left = left.as_ref(); // do we need clone???
            let right = right.as_ref();
            return Some((left, right));
        }
        None
    }
}

impl <'l, L: Ord, R: Ord> IntoIterator for &'l TwoWayMap<L, R> {
    type Item = (&'l L, &'l R);

    type IntoIter = RefIter<'l, L, R>;

    fn into_iter(self) -> Self::IntoIter {
        RefIter::new(self)
    }
    
}



#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct SomeStruct {
    pub(crate) a: i32,
    pub(crate) b: i32,
}

#[test]
pub(crate) fn test_new() {
    let map = TwoWayMap::<i32, i32>::new();
    assert_eq!(map.len(), 0);
}

#[test]
pub(crate) fn test_new_no_copy_trait() {
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert_eq!(map.len(), 0);

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);

    assert_eq!(map.len(), 1);
}

#[test]
pub(crate) fn test_different_types_new(){
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

#[test]
pub(crate) fn test_is_empyty() {
    let mut map = TwoWayMap::<i32, i32>::new();
    assert!(map.is_empty());

    map.insert(1, 2);
    assert!(!map.is_empty());

    map.insert(3, 4);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[test]
pub(crate) fn test_is_empty_some_struct() {
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert!(map.is_empty());

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[test]
pub(crate) fn test_is_empty_string() {
    let mut map = TwoWayMap::<String, String>::new();
    assert!(map.is_empty());

    let s1 = String::from("hello");
    let s2 = String::from("world");
    map.insert(s1.clone(), s2.clone());
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
}

#[test]
pub(crate) fn test_clear(){
    let mut map = TwoWayMap::<SomeStruct, i32>::new();
    assert!(map.is_empty());
    map.insert(SomeStruct { a: 1, b: 2 }, 3);
    map.insert(SomeStruct { a: 4, b: 5 }, 6);
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.left_to_right.len(), 0);
    assert_eq!(map.right_to_left.len(), 0);
}

#[test]
pub(crate) fn test_clear_string(){
    let mut map = TwoWayMap::<String, String>::new();
    assert!(map.is_empty());
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    assert!(!map.is_empty());

    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.left_to_right.len(), 0);
    assert_eq!(map.right_to_left.len(), 0);
}

#[test]
pub(crate) fn test_len() {
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

#[test]
pub(crate) fn test_len_string(){
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

#[test]
pub(crate) fn test_insert() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&2), Some(&1));
}

#[test]
pub(crate) fn test_insert_string(){
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&String::from("hello")), Some(&String::from("world")));
    assert_eq!(map.get_by_right(&String::from("world")), Some(&String::from("hello")));

    map.insert(String::from("foo"), String::from("bar"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get_by_left(&String::from("foo")), Some(&String::from("bar")));
    assert_eq!(map.get_by_right(&String::from("bar")), Some(&String::from("foo")));
}

#[test]
pub(crate) fn test_insert_no_overwrite() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(1, 3), Err((1, 3)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(3, 4), Ok(()));
    assert_eq!(map.len(), 2);
}


#[test]
pub(crate) fn test_insert_no_overwrite_string(){
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(String::from("hello"), String::from("foo")), Err((String::from("hello"), String::from("foo"))));
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert_no_overwrite(String::from("foo"), String::from("bar")), Ok(()));
    assert_eq!(map.len(), 2);
}

#[test]
pub(crate) fn test_remove_by_left(){
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

#[test]
pub(crate) fn test_remove_by_left_string(){
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(map.remove_by_left(&String::from("hello")), Some((String::from("hello"), String::from("world"))));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&String::from("hello")), None);
    assert_eq!(map.get_by_right(&String::from("world")), None);

    assert_eq!(map.get_by_left(&String::from("foo")), Some(&String::from("bar")));
}

#[test]
pub(crate) fn test_remove_by_right(){
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

#[test]
pub(crate) fn test_remove_by_right_string(){
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.len(), 2);
    assert_eq!(map.remove_by_right(&String::from("world")), Some((String::from("world"), String::from("hello"))));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&String::from("hello")), None);
    assert_eq!(map.get_by_right(&String::from("world")), None);

    assert_eq!(map.get_by_left(&String::from("foo")), Some(&String::from("bar")));
}

#[test]
pub(crate) fn test_remove_some_struct(){
    // by left
    let mut map = TwoWayMap::<SomeStruct, SomeStruct>::new();
    assert_eq!(map.len(), 0);

    let struct1 = SomeStruct { a: 1, b: 2 };
    let struct2 = SomeStruct { a: 3, b: 4 };
    map.insert(struct1, struct2);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&SomeStruct { a: 1, b: 2 }), Some(&SomeStruct { a: 3, b: 4 }));

    map.remove_by_left(&SomeStruct { a: 1, b: 2 });

    assert_eq!(map.len(), 0);
    assert_eq!(map.get_by_left(&SomeStruct { a: 1, b: 2 }), None);

    // by right
    let struct3 = SomeStruct { a: 5, b: 6 };
    let struct4 = SomeStruct { a: 7, b: 8 };
    map.insert(struct3, struct4);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get_by_left(&SomeStruct { a: 5, b: 6 }), Some(&SomeStruct { a: 7, b: 8 }));

    map.remove_by_right(&SomeStruct { a: 7, b: 8 });

    assert_eq!(map.len(), 0);
    assert_eq!(map.get_by_left(&SomeStruct { a: 5, b: 6 }), None);
}

#[test]
pub(crate) fn test_get(){
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

#[test]
pub(crate) fn test_get_string(){
    let mut map = TwoWayMap::<String, String>::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    assert_eq!(map.get_by_left(&String::from("hello")), Some(&String::from("world")));
    assert_eq!(map.get_by_right(&String::from("world")), Some(&String::from("hello")));
    assert_eq!(map.get_by_left(&String::from("foo")), Some(&String::from("bar")));
    assert_eq!(map.get_by_right(&String::from("bar")), Some(&String::from("foo")));
    assert_eq!(map.get_by_left(&String::from("baz")), None);
    assert_eq!(map.get_by_right(&String::from("qux")), None);
}

#[test]
pub(crate) fn test_contains(){
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

#[test]
pub(crate) fn test_contains_some_struct(){
    let mut map2 = TwoWayMap::new();
    let s1 = SomeStruct { a: 1, b: 2 };
    let s2 = SomeStruct { a: 3, b: 4 };

    map2.insert(s1, s2);
    assert!(!map2.contains_left(&SomeStruct { a: 5, b: 6 }));
    assert!(!map2.contains_right(&SomeStruct { a: 7, b: 8 }));
}

#[test]
pub(crate) fn test_contains_string(){
    let mut map4 = TwoWayMap::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    map4.insert(s1.clone(), s2.clone());
    assert!(map4.contains_left(&s1));
    assert!(map4.contains_right(&s2));
    assert!(!map4.contains_left(&String::from("foo")));
    assert!(!map4.contains_right(&String::from("bar")));
}
#[test]
pub(crate) fn test_pairs() {
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], (&1, &2));
    assert_eq!(pairs[1], (&3, &4));
}

#[test]
pub(crate) fn test_pairs_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[1], (&String::from("hello"), &String::from("world")));
    assert_eq!(pairs[0], (&String::from("foo"), &String::from("bar")));
}

#[test]
pub(crate) fn test_pairs_ascending(){
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

#[test]
pub(crate) fn test_pairs_are_not_consumed(){
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

#[test]
pub(crate) fn test_pairs_are_not_consumed_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    let pairs: Vec<_> = map.pairs().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[1], (&String::from("hello"), &String::from("world")));
    assert_eq!(pairs[0], (&String::from("foo"), &String::from("bar")));

    // Check that the map is still intact
    assert_eq!(map.len(), 2);
    assert_eq!(map.get_by_left(&String::from("hello")), Some(&String::from("world")));
    assert_eq!(map.get_by_right(&String::from("bar")), Some(&String::from("foo")));
}

#[test]
pub(crate) fn test_left_values(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 20);
    map.insert(100, 0);
    map.insert(3, 4);

    let left_values: Vec<_> = map.left_values().collect();
    assert_eq!(left_values.len(), 4);
    assert_eq!(left_values, vec![&1, &3, &5, &100]);
}

#[test]
pub(crate) fn test_left_values_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let left_values: Vec<_> = map.left_values().collect();
    assert_eq!(left_values.len(), 3);
    assert_eq!(left_values, vec![&String::from("baz"), &String::from("foo"), &String::from("hello"), ]);
}

#[test]
pub(crate) fn test_right_values(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 20);
    map.insert(100, 0);
    map.insert(3, 4);
    let right_values: Vec<_> = map.right_values().collect();
    assert_eq!(right_values.len(), 4);
    assert_eq!(right_values, vec![&0, &2, &4, &20]);
}

#[test]
pub(crate) fn test_right_values_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let right_values: Vec<_> = map.right_values().collect();
    assert_eq!(right_values.len(), 3);
    assert_eq!(right_values, vec![&String::from("bar"), &String::from("qux"), &String::from("world")]);
}

#[test]
pub(crate) fn test_range() {
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

#[test]
pub(crate) fn test_range_literal_string(){
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

#[test]
pub(crate) fn test_range_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let start = "a".to_string();
    let end = "foo".to_string();
    let range: Vec<_> = map.left_range(&start..=&end).collect();
    assert_eq!(range.len(), 2);
    assert_eq!(range, vec![(&String::from("baz"), &String::from("qux")), (&String::from("foo"), &String::from("bar"))]);

    let range: Vec<_> = map.left_range(&start..&end).collect();
    assert_eq!(range.len(), 1);
    assert_eq!(range, vec![(&String::from("baz"), &String::from("qux"))]);
}

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

#[test]
pub(crate) fn test_retain() {
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

#[test]
pub(crate) fn test_retain_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));
    map.insert(String::from("quux"), String::from("corge"));

    assert_eq!(map.len(), 4);

    map.retain(|left, right| left == "foo" || right == "qux");

    assert_eq!(map.len(), 2);
    assert_eq!(map.get_by_left(&String::from("foo")), Some(&String::from("bar")));
    assert_eq!(map.get_by_right(&String::from("qux")), Some(&String::from("baz")));
}

#[test]
pub(crate) fn test_clone() {
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

#[test]
pub(crate) fn test_clone_string(){
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

#[test]
pub(crate) fn test_extend(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);

    let new_pairs = vec![(5, 6), (7, 8)];
    map.extend(new_pairs);

    assert_eq!(map.len(), 4);
    assert_eq!(map.get_by_left(&5), Some(&6));
    assert_eq!(map.get_by_right(&8), Some(&7));
}

#[test]
pub(crate) fn test_extend_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));

    let new_pairs = vec![(String::from("baz"), String::from("qux")), (String::from("quux"), String::from("corge"))];
    map.extend(new_pairs);

    assert_eq!(map.len(), 4);
    assert_eq!(map.get_by_left(&String::from("baz")), Some(&String::from("qux")));
    assert_eq!(map.get_by_right(&String::from("corge")), Some(&String::from("quux")));
}

#[test]
pub(crate) fn test_from_iterator(){
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    let map: TwoWayMap<i32, i32> = TwoWayMap::from_iter(pairs);

    assert_eq!(map.len(), 3);
    assert_eq!(map.get_by_left(&1), Some(&2));
    assert_eq!(map.get_by_right(&4), Some(&3));
    assert_eq!(map.get_by_left(&5), Some(&6));
}

#[test]
pub(crate) fn test_from_iterator_string(){
    let pairs = vec![(String::from("hello"), String::from("world")), (String::from("foo"), String::from("bar"))];
    let map: TwoWayMap<String, String> = TwoWayMap::from_iter(pairs);

    assert_eq!(map.len(), 2);
    assert_eq!(map.get_by_left(&String::from("hello")), Some(&String::from("world")));
    assert_eq!(map.get_by_right(&String::from("bar")), Some(&String::from("foo")));
}

#[test]
pub(crate) fn test_into_iter(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 6);
    map.insert(3, 4);

    let mut iter: IntoIter<i32, i32> = map.into_iter();
    assert_eq!(iter.next(), Some((1, 2)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), Some((5, 6)));

    assert_eq!(iter.next(), None);
}

#[test]
pub(crate) fn test_into_iter_string(){
    let mut map = TwoWayMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.insert(String::from("foo"), String::from("bar"));
    map.insert(String::from("baz"), String::from("qux"));

    let mut iter: IntoIter<String, String> = map.into_iter();
    assert_eq!(iter.next(), Some((String::from("baz"), String::from("qux"))));
    assert_eq!(iter.next(), Some((String::from("foo"), String::from("bar"))));
    assert_eq!(iter.next(), Some((String::from("hello"), String::from("world"))));

    assert_eq!(iter.next(), None);
}

#[test]
pub(crate) fn test_into_iter_with_clone(){
    let mut map1 = TwoWayMap::new();
    map1.insert(1, 2);
    map1.insert(5, 6);
    map1.insert(3, 4);

    let map = map1.clone();

    let mut iter: IntoIter<i32, i32> = map.into_iter();
    assert_eq!(iter.next(), Some((1, 2)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), Some((5, 6)));

    assert_eq!(iter.next(), None);
}

#[test]
pub(crate) fn test_into_iter_string_with_clone(){
    let mut map1 = TwoWayMap::new();
    map1.insert(String::from("hello"), String::from("world"));
    map1.insert(String::from("foo"), String::from("bar"));
    map1.insert(String::from("baz"), String::from("qux"));

    let map = map1.clone();

    let mut iter: IntoIter<String, String> = map.into_iter();
    assert_eq!(iter.next(), Some((String::from("baz"), String::from("qux"))));
    assert_eq!(iter.next(), Some((String::from("foo"), String::from("bar"))));
    assert_eq!(iter.next(), Some((String::from("hello"), String::from("world"))));

    assert_eq!(iter.next(), None);
}

#[test]
pub(crate) fn test_into_iter_in_for_loop(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);


    for x in map.into_iter() {
        println!("x: {:?}", x);
    };

    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    for x in map {
        println!("x: {:?}", x);
    };

    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(5, 6);
    map.insert(3, 4);

}

#[test]
pub(crate) fn test_ref_into_iter(){
    let mut map = TwoWayMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);

    for x in &map {
        println!("x: {:?}", x);
    };

}


