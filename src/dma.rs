pub unsafe trait DmaAllocator {
    fn dma_addr(&self) -> *mut u8;
    fn dma_size(&self) -> usize;
    fn idle(&self) -> bool;
    fn allocated_size(&self) -> usize;
    fn free_size(&self) -> usize;
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError>;
    unsafe fn free(&mut self, layout: Layout) -> Result<(), AllocError>;
}
