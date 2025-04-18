use std::borrow::Borrow;
use std::collections;
use std::{fmt::Debug, ops::RangeBounds};

use crate::mem::{Rc, wrap_range, wrap_ref};

#[derive(Debug)]
pub struct TwoWayMap<L, R> {
    left_to_right: std::collections::BTreeMap<Rc<L>, Rc<R>>,
    right_to_left: std::collections::BTreeMap<Rc<R>, Rc<L>>,
}

impl<L, R> TwoWayMap<L, R> {
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
}

impl<L: Ord, R: Ord> TwoWayMap<L, R> {
    pub fn insert(&mut self, left: L, right: R) {
        // Check if left already exists
        if let Some(existing_right) = self.left_to_right.get(wrap_ref(&left)) {
            self.right_to_left.remove(existing_right);
        }
        // Check if right already exists
        if let Some(existing_left) = self.right_to_left.get(wrap_ref(&right)) {
            self.left_to_right.remove(existing_left);
        }

        let left = Rc::new(left);
        let right = Rc::new(right);

        self.left_to_right.insert(left.clone(), right.clone());
        self.right_to_left.insert(right, left);
    }

    pub fn insert_no_overwrite(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        //Check if left or right already exists
        if self.left_to_right.contains_key(wrap_ref(&left))
            || self.right_to_left.contains_key(wrap_ref(&right))
        {
            return Err((left, right));
        }
        self.insert(left, right);
        Ok(())
    }

    pub fn remove_by_left<Q>(&mut self, left: &Q) -> Option<(L, R)>
    where
        L: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(right) = self.left_to_right.remove(wrap_ref(left)) {
            let left = self.right_to_left.remove(&right).unwrap();

            // Convert Rc to L and R
            let left = std::rc::Rc::try_unwrap(left.0).ok().unwrap();
            let right = std::rc::Rc::try_unwrap(right.0).ok().unwrap();

            let pair = (left, right);

            return Some(pair);
        }
        None
    }

    pub fn remove_by_right<Q>(&mut self, right: &Q) -> Option<(R, L)>
    where
        R: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(left) = self.right_to_left.remove(wrap_ref(right)) {
            let right = self.left_to_right.remove(&left).unwrap().clone();
            // Convert Rc to L and R
            let left = std::rc::Rc::try_unwrap(left.0).ok().unwrap();
            let right = std::rc::Rc::try_unwrap(right.0).ok().unwrap();

            let pair = (right, left);

            return Some(pair);
        }
        None
    }

    pub fn get_by_left<Q>(&self, left: &Q) -> Option<&R>
    where
        L: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(right) = self.left_to_right.get(wrap_ref(left)) {
            return Some(right.as_ref());
        }
        None
    }

    pub fn get_by_right<Q>(&self, right: &Q) -> Option<&L>
    where
        R: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if let Some(left) = self.right_to_left.get(wrap_ref(right)) {
            return Some(left.as_ref());
        }
        None
    }

    pub fn contains_left<Q>(&self, left: &Q) -> bool
    where
        L: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.left_to_right.contains_key(wrap_ref(left))
    }

    pub fn contains_right<Q>(&self, right: &Q) -> bool
    where
        R: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.right_to_left.contains_key(wrap_ref(right))
    }
}

impl<L, R> TwoWayMap<L, R> {
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
}

impl<L: Ord, R: Ord> TwoWayMap<L, R> {
    pub fn left_range<T>(&self, range: T) -> impl Iterator<Item = (&L, &R)>
    where
        T: RangeBounds<L>,
    {
        self.left_to_right
            .range(wrap_range(&range))
            .map(|(left, right)| (left.as_ref(), right.as_ref()))
    }

    pub fn right_range<T>(&self, range: T) -> impl Iterator<Item = (&R, &L)>
    where
        T: RangeBounds<R>,
    {
        self.right_to_left
            .range(wrap_range(&range))
            .map(|(right, left)| (right.as_ref(), left.as_ref()))
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&L, &R) -> bool,
    {
        self.left_to_right
            .retain(|left, right| f(left.0.as_ref(), right.0.as_ref()));

        self.right_to_left
            .retain(|right, left| f(left.0.as_ref(), right.0.as_ref()));
    }
}

impl<L, R> Default for TwoWayMap<L, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Ord, R: Ord> Clone for TwoWayMap<L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        let mut other = TwoWayMap::new();
        for (left, right) in self.left_to_right.iter() {
            let left = left.0.as_ref().clone();
            let right = right.0.as_ref().clone();
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

pub struct IntoIter<L, R> {
    left_to_right_iter: std::collections::btree_map::IntoIter<Rc<L>, Rc<R>>,
}

impl<L, R> IntoIter<L, R> {
    fn new(map: TwoWayMap<L, R>) -> Self {
        Self {
            left_to_right_iter: map.left_to_right.into_iter(),
        }
    }
}

impl<L, R> Iterator for IntoIter<L, R> {
    type Item = (L, R);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left, right)) = self.left_to_right_iter.next() {
            let left = std::rc::Rc::try_unwrap(left.0).ok().unwrap();
            let right = std::rc::Rc::try_unwrap(right.0).ok().unwrap();

            return Some((left, right));
        }
        None
    }
}

impl<L, R> IntoIterator for TwoWayMap<L, R> {
    type Item = (L, R);

    type IntoIter = IntoIter<L, R>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct RefIter<'l, L, R> {
    iter: collections::btree_map::Iter<'l, Rc<L>, Rc<R>>,
}

impl<'l, L, R> RefIter<'l, L, R> {
    fn new(map_ref: &'l TwoWayMap<L, R>) -> Self {
        Self {
            iter: map_ref.left_to_right.iter(),
        }
    }
}

impl<'l, L, R> Iterator for RefIter<'l, L, R> {
    type Item = (&'l L, &'l R);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left, right)) = self.iter.next() {
            let left = left.as_ref();
            let right = right.as_ref();
            return Some((left, right));
        }
        None
    }
}

impl<'l, L, R> IntoIterator for &'l TwoWayMap<L, R> {
    type Item = (&'l L, &'l R);

    type IntoIter = RefIter<'l, L, R>;

    fn into_iter(self) -> Self::IntoIter {
        RefIter::new(self)
    }
}
