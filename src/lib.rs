use std::ops::{Bound, RangeBounds};
use std::pin::Pin;

pub fn is_included<T>(bound: &Bound<T>) -> bool {
    match bound {
        Bound::Included(_) => true,
        _ => false,
    }
}
pub fn is_excluded<T>(bound: &Bound<T>) -> bool {
    match bound {
        Bound::Excluded(_) => true,
        _ => false,
    }
}
pub fn is_unbounded<T>(bound: &Bound<T>) -> bool {
    match bound {
        Bound::Unbounded => true,
        _ => false,
    }
}
pub fn map<T, U, F: FnOnce(T) -> U>(bound: Bound<T>, f: F) -> Bound<U> {
    match bound {
        Bound::Excluded(b) => Bound::Excluded(f(b)),
        Bound::Included(b) => Bound::Included(f(b)),
        Bound::Unbounded => Bound::Unbounded,
    }
}
pub fn as_ref<U: ?Sized, T: AsRef<U>>(bound: &Bound<T>) -> Bound<&U> {
    match bound {
        Bound::Excluded(b) => Bound::Excluded(b.as_ref()),
        Bound::Included(b) => Bound::Included(b.as_ref()),
        Bound::Unbounded => Bound::Unbounded,
    }
}
pub fn as_mut<U: ?Sized, T: AsMut<U>>(bound: &mut Bound<T>) -> Bound<&mut U> {
    match bound {
        Bound::Excluded(b) => Bound::Excluded(b.as_mut()),
        Bound::Included(b) => Bound::Included(b.as_mut()),
        Bound::Unbounded => Bound::Unbounded,
    }
}
pub fn as_pin_ref<'a, U: ?Sized, T: AsRef<U>>(bound: Pin<&'a Bound<T>>) -> Bound<Pin<&'a U>> {
    unsafe { map(as_ref(Pin::get_ref(bound)), |x| Pin::new_unchecked(x)) }
}
pub fn as_pin_mut<'a, U: ?Sized, T: AsMut<U>>(
    bound: Pin<&'a mut Bound<T>>,
) -> Bound<Pin<&'a mut U>> {
    unsafe {
        map(as_mut(Pin::get_unchecked_mut(bound)), |x| {
            Pin::new_unchecked(x)
        })
    }
}
pub fn unwrap<T>(bound: Bound<T>) -> T {
    expect(
        bound,
        "called `rangetools::unwrap()` on a `Unbounded` value",
    )
}
pub fn unwrap_or<T>(bound: Bound<T>, def: T) -> T {
    match bound {
        Bound::Excluded(b) => b,
        Bound::Included(b) => b,
        Bound::Unbounded => def,
    }
}
pub fn unwrap_or_else<T, F>(bound: Bound<T>, f: F) -> T
where
    F: FnOnce() -> T,
{
    match bound {
        Bound::Excluded(b) => b,
        Bound::Included(b) => b,
        Bound::Unbounded => f(),
    }
}
pub fn expect<T>(bound: Bound<T>, msg: &str) -> T {
    match bound {
        Bound::Excluded(b) => b,
        Bound::Included(b) => b,
        Bound::Unbounded => panic!("{}", msg),
    }
}
pub fn cloned<T: Clone>(bound: Bound<&T>) -> Bound<T> {
    map(bound, |t| t.clone())
}
pub fn cloned_mut<T: Clone>(bound: Bound<&mut T>) -> Bound<T> {
    map(bound, |t| t.clone())
}
pub fn copied<T: Copy>(bound: Bound<&T>) -> Bound<T> {
    map(bound, |&t| t)
}
pub fn copied_mut<T: Copy>(bound: Bound<&mut T>) -> Bound<T> {
    map(bound, |&mut t| t)
}

pub struct MyBound<T>(Bound<T>);
impl<T> MyBound<T> {
    pub fn is_included(&self) -> bool {
        is_included(&self.0)
    }
    pub fn is_excluded(&self) -> bool {
        is_excluded(&self.0)
    }
    pub fn is_unbounded(&self) -> bool {
        is_unbounded(&self.0)
    }
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MyBound<U> {
        MyBound(map(self.0, f))
    }
    pub fn as_ref<U: ?Sized>(&self) -> MyBound<&U>
    where
        T: AsRef<U>,
    {
        MyBound(as_ref(&self.0))
    }
    pub fn as_mut<U: ?Sized>(&mut self) -> MyBound<&mut U>
    where
        T: AsMut<U>,
    {
        MyBound(as_mut(&mut self.0))
    }
    pub fn as_pin_ref<'a, U: ?Sized>(self: Pin<&'a MyBound<T>>) -> MyBound<Pin<&'a U>>
    where
        T: AsRef<U>,
    {
        MyBound(unsafe { map(as_ref(&Pin::get_ref(self).0), |x| Pin::new_unchecked(x)) })
    }
    pub fn as_pin_mut<'a, U: ?Sized>(self: Pin<&'a mut MyBound<T>>) -> MyBound<Pin<&'a mut U>>
    where
        T: AsMut<U>,
    {
        MyBound(unsafe {
            map(as_mut(&mut Pin::get_unchecked_mut(self).0), |x| {
                Pin::new_unchecked(x)
            })
        })
    }
    pub fn unwrap(self) -> T {
        unwrap(self.0)
    }
    pub fn unwrap_or(self, def: T) -> T {
        unwrap_or(self.0, def)
    }
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unwrap_or_else(self.0, f)
    }
    pub fn expect(self, msg: &str) -> T {
        expect(self.0, msg)
    }
}
impl<'a, T> MyBound<&'a T> {
    pub fn cloned(self) -> MyBound<T>
    where
        T: Clone,
    {
        MyBound(cloned(self.0))
    }
    pub fn copied(self) -> MyBound<T>
    where
        T: Copy,
    {
        MyBound(copied(self.0))
    }
}
impl<'a, T> MyBound<&'a mut T> {
    pub fn cloned(self) -> MyBound<T>
    where
        T: Clone,
    {
        MyBound(cloned_mut(self.0))
    }
    pub fn copied(self) -> MyBound<T>
    where
        T: Copy,
    {
        MyBound(copied_mut(self.0))
    }
}

pub trait MyRangeBounds<T: ?Sized>: RangeBounds<T> {
    fn start_bound(&self) -> MyBound<&T> {
        MyBound(RangeBounds::start_bound(self))
    }
    fn end_bound(&self) -> MyBound<&T> {
        MyBound(RangeBounds::end_bound(self))
    }
}
impl<T, U: RangeBounds<T>> MyRangeBounds<T> for U {}

impl<T> Into<Bound<T>> for MyBound<T> {
    fn into(self) -> Bound<T> {
        self.0
    }
}

impl<T> From<Bound<T>> for MyBound<T> {
    fn from(src: Bound<T>) -> MyBound<T> {
        MyBound(src)
    }
}

#[test]
fn test_as_ref() {
    let s: MyBound<String> = Bound::Included("abcd".to_owned()).into();
    let u: &[u8] = s.as_ref().unwrap();
    assert_eq!(u, b"abcd");
}
