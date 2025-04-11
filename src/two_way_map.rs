use std::collections;
use std::{fmt::Debug, ops::RangeBounds, ops::Bound::Included, rc::Rc};



#[derive(Debug, Default)]
pub struct TwoWayMap<L, R> {
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

pub struct IntoIter<L,R>{
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


pub struct RefIter<'l, L, R>{
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
