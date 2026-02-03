
use crate::*;

/// Trait for things that can be returned by a judge. Implemented for `bool` (the most common return type) and `Option<T>` (useful when you want to pass a simple payload of `T` to the inspector and penalizer).
///
/// After the Verdict is returned by the judge, Clubs will process it a little to determine what to do. First, it calls the `.is_accept()` method to determine whether the expression is being accepted or rejected.
///
///   - For `bool`, `true` returns `true` and `false` returns `false`.
///   - For `Option<T>`, `Some(T)` returns `true` and `None` returns `false`.
///
/// If `.is_accept()` returns true, Clubs will call the `.wrap()` method to construct the payload. This method accepts an [`Expression`] and returns a payload consisting of the expression and possibly some extra data — each type that implements Verdict gets to decide what exactly it returns here.
///
///   - For `bool`, the expression is left alone and returned as the payload.
///   - For `Option<T>`, the expression is wrapped in a [`Solution`] struct, which has a field to store the expression and a field to store the `T`.
///
/// The payload is passed to the inspector and penalizer by reference, and then returned to the user by value in the final [`Vec`] that the [`Searcher`] returns.

pub trait Verdict<N: Number, const C: usize> : Send {
    type Wrapper;

    fn is_accept(&self) -> bool;
    fn wrap(self, expr: Expression<N, C>) -> Self::Wrapper;
}

#[non_exhaustive]
pub struct Solution<N: Number, const C: usize, T> {
    pub expr: Expression<N, C>,
    pub data: T,
}

impl<N: Number, const C: usize> Verdict<N, C> for bool {
    type Wrapper = Expression<N, C>;

    fn is_accept(&self) -> bool {*self}
    fn wrap(self, expr: Expression<N, C>) -> Self::Wrapper {expr}
}

impl<N: Number, const C: usize, T: Send> Verdict<N, C> for Option<T> {
    type Wrapper = Solution<N, C, T>;

    fn is_accept(&self) -> bool {self.is_some()}
    fn wrap(self, expr: Expression<N, C>) -> Self::Wrapper {Solution {expr, data: self.unwrap()}}
}

