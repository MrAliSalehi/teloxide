#[cfg(feature = "frunk")]
use frunk::{from_generic, generic::Generic, hlist::HAppender, into_generic};

#[cfg(feature = "frunk")]
pub trait UpState: Sized {
    fn up<T1, T1Repr, F>(src: T1, field: F) -> Self
    where
        T1: Generic<Repr = T1Repr>,
        Self: Generic<Repr = <T1Repr as HAppender<F>>::Output>,
        T1Repr: HAppender<F>,
    {
        from_generic(into_generic(src).append(field))
    }
}

impl<T2> UpState for T2 {}
