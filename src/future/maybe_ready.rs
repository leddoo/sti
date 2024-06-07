use crate::future::{Future, Poll, Context, ready_or_yield};
use crate::pin::Pin;


pub enum MaybeReady<F: Future> {
    Taken,
    Pending(F),
    Ready(F::Output),
}

impl<F: Future> MaybeReady<F> {
    #[inline]
    pub fn new(f: F) -> MaybeReady<F> {
        MaybeReady::Pending(f)
    }

    #[inline]
    pub fn take(&mut self) -> Option<F::Output> {
        if let MaybeReady::Ready(_) = self {
            let MaybeReady::Ready(r) = crate::mem::replace(self, MaybeReady::Taken) else { unreachable!() };
            return Some(r);
        }
        None
    }
}

impl<F: Future> Future for MaybeReady<F> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this {
            MaybeReady::Taken => unreachable!(),

            MaybeReady::Pending(f) => {
                let r = ready_or_yield!(unsafe { Pin::new_unchecked(f) }.poll(cx));
                *this = MaybeReady::Ready(r);
                Poll::Ready(())
            }

            MaybeReady::Ready(_) => Poll::Ready(()),
        }
    }
}

