use core::arch::asm;

#[derive(Copy, Clone)]
struct AnyCPUID {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl core::fmt::Debug for AnyCPUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "AnyCPUID {{ eax: 0x{:016X}, ebx: 0x{:016X}, ecx: 0x{:016X}, edx: 0x{:016X} }}",
            self.eax, self.ebx, self.ecx, self.edx
        )
    }
}

pub struct BasicCPUIDInfo {
    pub max_basic_fn: u32,
    pub manufacturer: [char; 12],
}

impl core::fmt::Debug for BasicCPUIDInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "BasicCPUIDInfo {{ max_basic_fn: 0x{:016X}, manufacturer: \"",
            self.max_basic_fn
        )?;
        for c in &self.manufacturer {
            write!(f, "{}", c)?;
        }
        write!(f, "\" }}")?;

        Ok(())
    }
}

pub struct AdvancedCPUIDInfo {
    pub brand_idx: u8,
    pub extended_family: u8,
    pub extended_model: u8,
    pub type_: u8,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
}

impl core::fmt::Debug for AdvancedCPUIDInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "AdvancedCPUIDInfo {{ brand_idx: 0x{:01X}, extended_family: 0x{:02X}, extended_model: 0x{:01X}, type: 0x{:01X}, family: 0x{:01X}, model: 0x{:01X}, stepping: 0x{:01X}",
                       self.brand_idx, self.extended_family, self.extended_model, self.type_, self.family, self.model, self.stepping
        )?;

        if self.family == 6 || self.family == 15 {
            write!(f, " (")?;

            let a = self.extended_family << 4 + self.family;

            write!(f, "family: {:02}, ", a)?;
        }

        write!(f, " }}")
    }
}

fn string_from_regs(m0: u32, m1: u32, m2: u32) -> [char; 12] {
    let mut data = ['\0'; 12];
    data[0] = (((m0 >> 0) & 0xFF) as u8) as char;
    data[1] = (((m0 >> 8) & 0xFF) as u8) as char;
    data[2] = (((m0 >> 16) & 0xFF) as u8) as char;
    data[3] = (((m0 >> 24) & 0xFF) as u8) as char;
    data[4] = (((m1 >> 0) & 0xFF) as u8) as char;
    data[5] = (((m1 >> 8) & 0xFF) as u8) as char;
    data[6] = (((m1 >> 16) & 0xFF) as u8) as char;
    data[7] = (((m1 >> 24) & 0xFF) as u8) as char;
    data[8] = (((m2 >> 0) & 0xFF) as u8) as char;
    data[9] = (((m2 >> 8) & 0xFF) as u8) as char;
    data[10] = (((m2 >> 16) & 0xFF) as u8) as char;
    data[11] = (((m2 >> 24) & 0xFF) as u8) as char;

    data
}

fn do_cpuid(mut eax: u32) -> AnyCPUID {
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;

    unsafe {
        asm!(
            "cpuid",

            inlateout("eax") eax => eax,
            inlateout("ebx") 0 => ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
        );
    }

    AnyCPUID { eax, ebx, ecx, edx }
}

pub fn basic_cpuid() -> BasicCPUIDInfo {
    let AnyCPUID {
        eax,
        ebx: m0,
        edx: m1,
        ecx: m2,
    } = do_cpuid(0);

    let manufacturer = string_from_regs(m0, m1, m2);

    BasicCPUIDInfo {
        max_basic_fn: eax,
        manufacturer,
    }
}

pub fn advanced_cpuid() -> AdvancedCPUIDInfo {
    let AnyCPUID { eax, ebx, .. } = do_cpuid(1);

    AdvancedCPUIDInfo {
        brand_idx: (ebx & 0xFF) as u8,
        extended_family: ((eax >> 20) & 0xFF) as u8,
        extended_model: ((eax >> 16) & 0xF) as u8,
        type_: ((eax >> 12) & 0x3) as u8,
        family: ((eax >> 8) & 0xF) as u8,
        model: ((eax >> 4) & 0xF) as u8,
        stepping: (eax & 0xF) as u8,
    }
}
