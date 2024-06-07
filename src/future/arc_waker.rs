use crate::mem::{ManuallyDrop, PhantomData};
use crate::future::{Waker, RawWaker, RawWakerVTable};
use crate::sync::Arc;


pub trait ArcWake: Send + Sync {
    fn wake(this: Arc<Self>) {
        Self::wake_by_ref(&this)
    }

    fn wake_by_ref(this: &Arc<Self>);
}


pub struct ArcWaker<'a> {
    waker: ManuallyDrop<Waker>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ArcWaker<'a> {
    #[inline]
    pub fn new<W: ArcWake>(w: &'a Arc<W>) -> ArcWaker<'a> {
        let raw = RawWaker::new(w.as_ptr().cast(), arc_waker_vtable::<W>());
        return ArcWaker {
            waker: ManuallyDrop::new(unsafe { Waker::from_raw(raw) }),
            phantom: PhantomData,
        };
    }
}

impl<'a> core::ops::Deref for ArcWaker<'a> {
    type Target = Waker;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.waker
    }
}


const fn arc_waker_vtable<W: ArcWake>() -> &'static RawWakerVTable {
    &RawWakerVTable::new(
        // clone.
        |this| {
            let arc = ManuallyDrop::new(unsafe { Arc::from_raw(this.cast::<W>()) });
            let _clone = ManuallyDrop::new((&*arc).clone());
            return RawWaker::new(this, arc_waker_vtable::<W>())
        },
        // wake.
        |this| {
            let arc = unsafe { Arc::from_raw(this.cast::<W>()) };
            ArcWake::wake(arc);
        },
        // wake_by_ref.
        |this| {
            let arc = ManuallyDrop::new(unsafe { Arc::from_raw(this.cast::<W>()) });
            ArcWake::wake_by_ref(&arc);
        },
        // drop.
        |this| {
            drop(unsafe { Arc::from_raw(this.cast::<W>()) });
        })
}

