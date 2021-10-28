use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn calloc(num: usize, size: usize) -> *mut core::ffi::c_void;
    fn realloc(ptr: *mut core::ffi::c_void, new_size: usize) -> *mut core::ffi::c_void;
}

struct CrtAllocator;

#[global_allocator]
static ALLOCATOR: CrtAllocator = CrtAllocator {};

unsafe impl GlobalAlloc for CrtAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size() + layout.align()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr as *mut core::ffi::c_void) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { calloc(1, layout.size() + layout.align()) as *mut u8 }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { realloc(ptr as *mut core::ffi::c_void, new_size + layout.align()) as *mut u8 }
    }
}

#[alloc_error_handler]
pub fn rust_oom(_: core::alloc::Layout) -> ! {
    loop {}
}
