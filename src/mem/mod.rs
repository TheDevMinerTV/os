mod area_frame_alloc;

use core::fmt::Display;

pub use area_frame_alloc::AreaFrameAllocator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const PAGE_SIZE: usize = 4096;

impl Display for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{idx={},size={PAGE_SIZE}b,start=0x{:08X},end=0x{:08X}}}",
            self.number,
            self.start_address(),
            self.end_address()
        )
    }
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    pub fn end_address(&self) -> usize {
        ((self.number + 1) * PAGE_SIZE) - 1
    }
}

pub trait FrameAllocator {
    fn allocate(&mut self) -> Option<Frame>;
    fn deallocate(&mut self, frame: Frame);
}
