use bytemuck::TransparentWrapper;
use std::{
    borrow::Borrow,
    ops::{Bound, RangeBounds},
};

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rc<T: ?Sized>(pub std::rc::Rc<T>);

impl<T> Rc<T> {
    pub fn new(val: T) -> Self {
        Self(std::rc::Rc::new(val))
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

// This implementation is not possible because it conflicts with `impl Borrow<T> for T`.
// That's why we have to wrap references in a custom type and implement Borrow for it
//
// impl<T> Borrow<T> for Ref<T> {
//     fn borrow(&self) -> &T {
//         self.0.as_ref()
//     }
// }

#[derive(TransparentWrapper, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[repr(transparent)]
pub struct Wrapper<T: ?Sized>(T);

pub fn wrap_ref<T: ?Sized>(value: &T) -> &Wrapper<T> {
    Wrapper::wrap_ref(value)
}

pub fn wrap_range<'l, T: ?Sized + 'l, R: RangeBounds<T>>(
    range: &'l R,
) -> impl RangeBounds<Wrapper<T>> {
    let start = range.start_bound();
    let end = range.end_bound();
    (wrap_bound(start), wrap_bound(end))
}

pub fn wrap_bound<T: ?Sized>(bound: Bound<&T>) -> Bound<&Wrapper<T>> {
    bound.map(|v| wrap_ref(v))
}

impl<Q: ?Sized, T: Borrow<Q>> Borrow<Wrapper<Q>> for Rc<T> {
    fn borrow(&self) -> &Wrapper<Q> {
        Wrapper::wrap_ref(self.0.as_ref().borrow())
    }
}
