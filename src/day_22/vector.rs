use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn repeat(elem: T) -> Self
    where
        T: Clone,
    {
        Self::try_from(vec![elem; N]).unwrap_or_else(|_| unreachable!())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn map<U, F>(&self, f: F) -> Vector<U, N>
    where
        F: FnMut(&T) -> U,
    {
        self.iter()
            .map(f)
            .collect_vec()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn map_into<U, F>(self, f: F) -> Vector<U, N>
    where
        F: FnMut(T) -> U,
    {
        self.into_iter()
            .map(f)
            .collect_vec()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn zip_with<U, V, F>(&self, other: &Vector<U, N>, f: F) -> Vector<V, N>
    where
        F: Fn(&T, &U) -> V,
    {
        self.iter()
            .zip(other.iter())
            .map(|(t, u)| f(t, u))
            .collect_vec()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn zip_with_into<U, V, F>(self, other: Vector<U, N>, f: F) -> Vector<V, N>
    where
        F: Fn(T, U) -> V,
    {
        self.into_iter()
            .zip(other.into_iter())
            .map(|(t, u)| f(t, u))
            .collect_vec()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn add_scalar<U>(self, rhs: U) -> Vector<T::Output, N>
    where
        T: Add<U>,
        U: Clone,
    {
        let mut ret = Vec::new();
        for t in self {
            ret.push(t + rhs.clone());
        }
        ret.try_into().unwrap_or_else(|_| unreachable!())
    }

    pub fn sub_scalar<U>(self, rhs: U) -> Vector<T::Output, N>
    where
        T: Sub<U>,
        U: Clone,
    {
        let mut ret = Vec::new();
        for t in self {
            ret.push(t - rhs.clone());
        }
        ret.try_into().unwrap_or_else(|_| unreachable!())
    }

    #[must_use]
    pub fn inf(&self, other: &Self) -> Self
    where
        T: Clone + Ord,
    {
        self.zip_with(other, |t1, t2| t1.min(t2).clone())
    }

    #[must_use]
    pub fn sup(&self, other: &Self) -> Self
    where
        T: Clone + Ord,
    {
        self.zip_with(other, |t1, t2| t1.max(t2).clone())
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(orig: [T; N]) -> Self {
        Vector(orig)
    }
}

impl<T, const N: usize> TryFrom<Vec<T>> for Vector<T, N> {
    type Error = Vec<T>;
    fn try_from(orig: Vec<T>) -> Result<Self, Self::Error> {
        <[T; N]>::try_from(orig).map(Vector::from)
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;
    type IntoIter = <&'a [T] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

impl<T, U, const N: usize> Add<Vector<U, N>> for Vector<T, N>
where
    T: Add<U>,
{
    type Output = Vector<T::Output, N>;
    fn add(self, rhs: Vector<U, N>) -> Self::Output {
        self.zip_with_into(rhs, |t, u| t + u)
    }
}

impl<T, U, const N: usize> Sub<Vector<U, N>> for Vector<T, N>
where
    T: Sub<U>,
{
    type Output = Vector<T::Output, N>;
    fn sub(self, rhs: Vector<U, N>) -> Self::Output {
        self.zip_with_into(rhs, |t, u| t - u)
    }
}
