use super::*;
use core::arch::asm;

pub struct PagingController {
    directory: &'static [PageDirectoryEntry; 1024],
}

impl PagingController {
    pub unsafe fn initialize_at(address: usize) -> Self {
        assert!(address % 4096 == 0);

        let directory = &mut *(address as *mut _);
        core::ptr::write_bytes(
            directory,
            0,
            core::mem::size_of::<[PageDirectoryEntry; 1024]>(),
        );

        Self { directory }
    }

    pub unsafe fn enable_paging(&self) {
        asm!(
            "mov cr3, {tbl}",
            "mov cr0, cr0 | 0x80000000",

            tbl = in(reg) self.directory.as_ptr()
        );
    }
}
