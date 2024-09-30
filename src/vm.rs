use core::alloc::{GlobalAlloc, Layout};

use crate::AllocError;
pub unsafe trait VirtualAllocator: GlobalAlloc {
    unsafe fn malloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError>;
    unsafe fn free(&mut self, ptr: *mut u8, layout: Layout) -> Result<(), AllocError>;
    fn allocated_size(&self)->usize;
    fn free_size(&self)->usize;
    fn total_size(&self)->usize;
}
