#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageDirectoryEntry {
    inner: u32,
}

impl PageDirectoryEntry {
    pub fn new(address: *mut PageTableEntry, user: bool, rw: bool, present: bool) -> Self {
        assert!((address as usize) % 4096 == 0);

        let ps = 0; // Always 4k, not 4M
        let accessed = false; // Initially not accessed
        let cache_disable = false;
        let write_through = false; // TODO: Do we need write through

        let inner = ((address as u32) & 0xFFFFF000)
            // 4 bit available
            | ((ps as u32) << 7)
            // 1 bit available
            | ((accessed as u32) << 5)
            | ((cache_disable as u32) << 4)
            | ((write_through as u32) << 3)
            | ((user as u32) << 2)
            | ((rw as u32) << 1)
            | (present as u32);

        unsafe { Self::new_raw(inner) }
    }

    pub unsafe fn new_raw(inner: u32) -> Self {
        Self { inner }
    }

    pub fn address(&self) -> *mut PageTableEntry {
        (self.inner & 0xFFFFF000) as *mut PageTableEntry
    }

    pub fn set_address(&mut self, address: *mut PageTableEntry) -> &mut Self {
        self.inner = (self.inner & !0xFFFFF000) | ((address as u32) & 0xFFFFF000);
        self
    }

    pub fn is_4m(&self) -> bool {
        (self.inner & (1 << 7)) != 0
    }

    pub fn set_4m(&mut self, is_4m: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 7)) | ((is_4m as u32) << 7);
        self
    }

    pub fn was_accessed(&self) -> bool {
        (self.inner & (1 << 5)) != 0
    }

    pub fn set_was_accessed(&mut self, was_accessed: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 5)) | ((was_accessed as u32) << 5);
        self
    }

    pub fn cache_disabled(&self) -> bool {
        (self.inner & (1 << 4)) != 0
    }

    pub fn set_cache_disabled(&mut self, cache_disabled: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 4)) | ((cache_disabled as u32) << 4);
        self
    }

    pub fn is_write_through(&self) -> bool {
        (self.inner & (1 << 3)) != 0
    }

    pub fn set_write_through(&mut self, is_write_through: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 3)) | ((is_write_through as u32) << 3);
        self
    }

    pub fn is_user(&self) -> bool {
        (self.inner & (1 << 2)) != 0
    }

    pub fn set_user(&mut self, is_user: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 2)) | ((is_user as u32) << 2);
        self
    }

    pub fn is_read_write(&self) -> bool {
        (self.inner & (1 << 1)) != 0
    }

    pub fn set_read_write(&mut self, is_read_write: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 1)) | ((is_read_write as u32) << 1);
        self
    }

    pub fn is_present(&self) -> bool {
        (self.inner & (1 << 0)) != 0
    }

    pub fn set_present(&mut self, is_present: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 0)) | ((is_present as u32) << 0);
        self
    }
}

impl core::fmt::Debug for PageDirectoryEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageDirectoryEntry4k")
            .field("address", &self.address())
            .field("is_4m", &self.is_4m())
            .field("was_accessed", &self.was_accessed())
            .field("cache_disabled", &self.cache_disabled())
            .field("is_write_through", &self.is_write_through())
            .field("is_user", &self.is_user())
            .field("is_read_write", &self.is_read_write())
            .field("is_present", &self.is_present())
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    inner: u32
}

impl PageTableEntry {
    pub fn new(address: usize, user: bool, rw: bool, present: bool) -> Self {
        let inner = 0;

        let global = false; // TODO: Do we need global
        let pat = 0; // Assume not supported
        let dirty = false; // Initially not dirty
        let accessed = false; // Initially not accessed
        let cache_disable = false;
        let write_through = false; // TODO: Do we need write through

        let inner = ((address as u32) & 0xFFFFF000)
            // 3 bit available
            | ((global as u32) << 8)
            | ((pat as u32) << 7)
            | ((dirty as u32) << 6)
            | ((accessed as u32) << 5)
            | ((cache_disable as u32) << 4)
            | ((write_through as u32) << 3)
            | ((user as u32) << 2)
            | ((rw as u32) << 1)
            | (present as u32);

        unsafe { Self::new_raw(inner) }
    }

    pub unsafe fn new_raw(inner: u32) -> Self {
        Self { inner }
    }

    pub fn address(&self) -> usize {
        (self.inner & 0xFFFFF000) as usize
    }

    pub fn set_address(&mut self, address: usize) -> &mut Self {
        self.inner = (self.inner & !0xFFFFF000) | ((address as u32) & 0xFFFFF000);
        self
    }

    pub fn is_global(&self) -> bool {
        (self.inner & (1 << 8)) != 0
    }

    pub fn set_global(&mut self, is_global: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 8)) | ((is_global as u32) << 8);
        self
    }

    pub fn is_4m(&self) -> bool {
        (self.inner & (1 << 7)) != 0
    }

    pub fn set_4m(&mut self, is_4m: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 7)) | ((is_4m as u32) << 7);
        self
    }

    pub fn is_dirty(&self) -> bool {
        (self.inner & (1 << 6)) != 0
    }

    pub fn set_dirty(&mut self, is_dirty: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 6)) | ((is_dirty as u32) << 6);
        self
    }

    pub fn was_accessed(&self) -> bool {
        (self.inner & (1 << 5)) != 0
    }

    pub fn set_was_accessed(&mut self, was_accessed: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 5)) | ((was_accessed as u32) << 5);
        self
    }

    pub fn cache_disabled(&self) -> bool {
        (self.inner & (1 << 4)) != 0
    }

    pub fn set_cache_disabled(&mut self, cache_disabled: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 4)) | ((cache_disabled as u32) << 4);
        self
    }

    pub fn is_write_through(&self) -> bool {
        (self.inner & (1 << 3)) != 0
    }

    pub fn set_write_through(&mut self, is_write_through: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 3)) | ((is_write_through as u32) << 3);
        self
    }

    pub fn is_user(&self) -> bool {
        (self.inner & (1 << 2)) != 0
    }

    pub fn set_user(&mut self, is_user: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 2)) | ((is_user as u32) << 2);
        self
    }

    pub fn is_read_write(&self) -> bool {
        (self.inner & (1 << 1)) != 0
    }

    pub fn set_read_write(&mut self, is_read_write: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 1)) | ((is_read_write as u32) << 1);
        self
    }

    pub fn is_present(&self) -> bool {
        (self.inner & (1 << 0)) != 0
    }

    pub fn set_present(&mut self, is_present: bool) -> &mut Self {
        self.inner = (self.inner & !(1 << 0)) | ((is_present as u32) << 0);
        self
    }
}
