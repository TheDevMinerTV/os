use bitflags::bitflags;
use core::arch::asm;

#[derive(Copy, Clone)]
pub struct AnyCPUID {
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
    pub manufacturer: CPUManufacturer,
}

impl BasicCPUIDInfo {
    pub fn read() -> BasicCPUIDInfo {
        let AnyCPUID {
            eax,
            ebx: m0,
            edx: m1,
            ecx: m2,
        } = do_cpuid(0);

        let manufacturer = string_from_regs(m0, m1, m2);

        BasicCPUIDInfo {
            max_basic_fn: eax,
            manufacturer: manufacturer.into(),
        }
    }
}

impl core::fmt::Debug for BasicCPUIDInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "BasicCPUIDInfo {{ max_basic_fn: 0x{:016X}, manufacturer: \"{}\" }}",
            self.max_basic_fn, self.manufacturer
        )
    }
}

pub struct ExtendedCPUIDInfo {
    pub max_extended_fn: u32,
    pub info_and_bits: Option<ExtendedInfoAndBits>,
    pub svm_revision: Option<u8>,
}

impl ExtendedCPUIDInfo {
    pub fn read() -> ExtendedCPUIDInfo {
        let AnyCPUID { eax, .. } = do_cpuid(0x80000000);

        let info_and_bits = if eax < 0x80000001 {
            None
        } else {
            Some(ExtendedCPUIDInfo::read_info_and_bits())
        };

        let svm_revision = if eax < 0x8000000A {
            None
        } else {
            Some(ExtendedCPUIDInfo::read_svm_revision())
        };

        ExtendedCPUIDInfo {
            max_extended_fn: eax,
            info_and_bits,
            svm_revision,
        }
    }

    fn read_svm_revision() -> u8 {
        let AnyCPUID { eax, .. } = do_cpuid(0x8000000A);

        (eax & 0xFF) as u8
    }

    fn read_info_and_bits() -> ExtendedInfoAndBits {
        let AnyCPUID { eax, ebx, ecx, edx } = do_cpuid(0x80000001);

        ExtendedInfoAndBits {
            ecx: ExtendedInfoAndBitsECX::from_bits_truncate(ecx),
            edx: ExtendedInfoAndBitsEDX::from_bits_truncate(edx),
        }
    }
}

impl core::fmt::Debug for ExtendedCPUIDInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Extended CPUID Info {{ max_extended_fn: 0x{:016X}, info_and_bits: {:?}, svm_revision: {:?} }}",
            self.max_extended_fn, self.info_and_bits, self.svm_revision
        )
    }
}

#[derive(Debug)]
pub struct ExtendedInfoAndBits {
    pub ecx: ExtendedInfoAndBitsECX,
    pub edx: ExtendedInfoAndBitsEDX,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExtendedInfoAndBitsECX: u32 {
        const fpu = 1 << 0;
        const vme = 1 << 1;
        const de = 1 << 2;
        const pse = 1 << 3;
        const tsc = 1 << 4;
        const msr = 1 << 5;
        const pae = 1 << 6;
        const mce = 1 << 7;
        const cx8 = 1 << 8;
        const apic = 1 << 9;
        const syscall = 1 << 11;
        const mtrr = 1 << 12;
        const mca = 1 << 13;
        const cmov = 1 << 14;
        const pat = 1 << 15;
        const pse36 = 1 << 16;
        const mp = 1 << 17;
        const nx = 1 << 19;
        const mmxext = 1 << 20;
        const mmx = 1 << 22;
        const fxsr = 1 << 23;
        const fxsr_opt = 1 << 24;
        const pdpe1gb = 1 << 25;
        const rdtscp = 1 << 26;
        const lm = 1 << 27;
        const _3dnowext = 1 << 29;
        const _3dnow = 1 << 30;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExtendedInfoAndBitsEDX: u32 {
        const lahf_lm = 1 << 30;
        const cmp_legacy = 1 << 1;
        const svm = 1 << 2;
        const extapic =1 << 3 ;
        const cr8_legacy = 1 << 3;
        const abm = 1 << 5;
        const sse4a = 1 << 6;
        const misalignsse = 1 << 7;
        const _3dnowprefetch = 1 << 8;
        const osvw = 1 << 9;
        const ibs = 1 << 10;
        const xop = 1 << 11;
        const skinit = 1 << 12;
        const wdt = 1 << 13;
        const lwp = 1 << 14;
        const fma4 = 1 << 16;
        const tce = 1 << 17;
        const nodeid_msr = 1 << 18;
        const tbm = 1 << 20;
        const topoext = 1 << 22;
        const perfctr_core = 1 << 23;
        const perfctr_nb = 1 << 24;
        const dbx = 1 << 26;
        const perftsc = 1 << 27;
        const pcx_l2i = 1 << 28;
        const monitorx = 1 << 29;
        const addr_mask_ext =1 << 30;
    }
}

pub struct CPUManufacturer([char; 12]);

impl core::fmt::Display for CPUManufacturer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl From<[char; 12]> for CPUManufacturer {
    fn from(c: [char; 12]) -> CPUManufacturer {
        CPUManufacturer(c)
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
