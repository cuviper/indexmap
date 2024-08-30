pub(crate) use self::inner::{do_alloc, Allocator, Global};

// No-defaults case.
// When building with default-features turned off and
// neither `nightly` nor `allocator-api2` is enabled,
// this will be used.
// Making it impossible to use any custom allocator with collections defined
// in this crate.
// Any crate in build-tree can enable `allocator-api2`,
// or `nightly` without disturbing users that don't want to use it.
mod inner {
    use crate::alloc::alloc::{alloc, dealloc, Layout};
    use core::ptr::NonNull;

    #[allow(clippy::missing_safety_doc)] // not exposed outside of this crate
    pub(crate) unsafe trait Allocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    }

    #[derive(Copy, Clone)]
    pub(crate) struct Global;

    unsafe impl Allocator for Global {
        #[inline]
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unsafe { NonNull::new(alloc(layout)).ok_or(()) }
        }
        #[inline]
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            dealloc(ptr.as_ptr(), layout);
        }
    }

    impl Default for Global {
        #[inline]
        fn default() -> Self {
            Global
        }
    }

    pub(crate) fn do_alloc<A: Allocator>(alloc: &A, layout: Layout) -> Result<NonNull<u8>, ()> {
        alloc.allocate(layout)
    }
}
