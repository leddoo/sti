pub use core::future::{Future, IntoFuture, Ready, ready, Pending, pending, PollFn, poll_fn};
pub use core::task::{Poll, Context, Waker, RawWaker, RawWakerVTable, ready as ready_or_yield};

mod maybe_ready;
mod arc_waker;

pub use maybe_ready::MaybeReady;
pub use arc_waker::{ArcWake, ArcWaker};


pub fn yield_now() -> impl Future<Output=()> + Unpin {
    let mut yielded = false;
    poll_fn(move |cx| {
        if !yielded {
            yielded = true;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        return Poll::Ready(())
    })
}


