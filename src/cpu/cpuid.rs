use bitflags::bitflags;
use core::arch::asm;

use crate::misc::cstring::fix_zeroterminated_string;

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

pub struct Basic {
    pub max_basic_fn: u32,
    pub manufacturer: Manufacturer,
    pub basic_info: Option<BasicInfo>,
}

impl Basic {
    pub fn read() -> Basic {
        let AnyCPUID {
            eax,
            ebx: m0,
            edx: m1,
            ecx: m2,
        } = do_cpuid(0);

        let manufacturer = string_from_regs(m0, m1, m2);

        Basic {
            max_basic_fn: eax,
            manufacturer: manufacturer.into(),
            basic_info: if eax < 1 {
                None
            } else {
                Some(Basic::read_basic_info())
            },
        }
    }

    fn read_basic_info() -> BasicInfo {
        let AnyCPUID { eax, ebx, .. } = do_cpuid(1);

        BasicInfo {
            brand_idx: (ebx & 0xFF) as u8,
            extended_family: ((eax >> 20) & 0xFF) as u8,
            extended_model: ((eax >> 16) & 0xF) as u8,
            type_: ((eax >> 12) & 0x3) as u8,
            family: ((eax >> 8) & 0xF) as u8,
            model: ((eax >> 4) & 0xF) as u8,
            stepping: (eax & 0xF) as u8,
        }
    }
}

impl core::fmt::Debug for Basic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ max_basic_fn: 0x{:016X}, manufacturer: {}, basic_info: {:?} }}",
            self.max_basic_fn, self.manufacturer, self.basic_info
        )
    }
}

pub struct BasicInfo {
    pub brand_idx: u8,
    pub extended_family: u8,
    pub extended_model: u8,
    pub type_: u8,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
}

impl core::fmt::Debug for BasicInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ brand_idx: 0x{:01X}, extended_family: 0x{:02X}, extended_model: 0x{:01X}, type: 0x{:01X}, family: 0x{:01X}, model: 0x{:01X}, stepping: 0x{:01X} }}",
                       self.brand_idx, self.extended_family, self.extended_model, self.type_, self.family, self.model, self.stepping
        )
    }
}

pub struct Extended {
    pub max_extended_fn: u32,
    pub vendor: Option<Manufacturer>,
    pub info_and_bits: Option<ExtendedInfoAndBits>,
    pub brand: Option<Brand>,
    pub svm_revision: Option<u8>,
}

impl Extended {
    pub fn read() -> Extended {
        let AnyCPUID { eax, ebx, ecx, edx } = do_cpuid(0x80000000);

        Extended {
            max_extended_fn: eax,
            vendor: if eax < 0x80000000 {
                None
            } else {
                Some(Manufacturer(string_from_regs(ebx, edx, ecx)))
            },
            info_and_bits: if eax < 0x80000001 {
                None
            } else {
                Some(Extended::read_info_and_bits())
            },
            brand: if eax < 0x80000004 {
                None
            } else {
                Some(Extended::read_brand())
            },
            svm_revision: if eax < 0x8000000A {
                None
            } else {
                Some(Extended::read_svm_revision())
            },
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

    fn read_brand() -> Brand {
        let AnyCPUID { eax, ebx, ecx, edx } = do_cpuid(0x80000002);
        let AnyCPUID {
            eax: eax2,
            ebx: ebx2,
            ecx: ecx2,
            edx: edx2,
        } = do_cpuid(0x80000003);
        let AnyCPUID {
            eax: eax3,
            ebx: ebx3,
            ecx: ecx3,
            edx: edx3,
        } = do_cpuid(0x80000004);

        Brand(string_from_4_regs(
            eax, ebx, ecx, edx, eax2, ebx2, ecx2, edx2, eax3, ebx3, ecx3, edx3,
        ))
    }
}

impl core::fmt::Debug for Extended {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ max_extended_fn: 0x{:016X}, vendor: {:?}, info_and_bits: {:?}, brand: {:?}, svm_revision: {:?} }}",
            self.max_extended_fn, self.vendor, self.info_and_bits, self.brand, self.svm_revision
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

pub struct Manufacturer([char; 12]);

impl core::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl core::fmt::Debug for Manufacturer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ manufacturer: {} }}", self)
    }
}

impl From<[char; 12]> for Manufacturer {
    fn from(c: [char; 12]) -> Manufacturer {
        Manufacturer(c)
    }
}

pub struct Brand([char; 48]);

impl core::fmt::Display for Brand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl core::fmt::Debug for Brand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ brand: {}}}", self)
    }
}

impl From<[char; 48]> for Brand {
    fn from(c: [char; 48]) -> Brand {
        Brand(c)
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

    fix_zeroterminated_string(&mut data);

    data
}

fn string_from_4_regs(
    m01: u32,
    m11: u32,
    m21: u32,
    m31: u32,
    m02: u32,
    m12: u32,
    m22: u32,
    m32: u32,
    m03: u32,
    m13: u32,
    m23: u32,
    m33: u32,
) -> [char; 48] {
    let mut data = ['\0'; 48];
    data[0] = (((m01 >> 0) & 0xFF) as u8) as char;
    data[1] = (((m01 >> 8) & 0xFF) as u8) as char;
    data[2] = (((m01 >> 16) & 0xFF) as u8) as char;
    data[3] = (((m01 >> 24) & 0xFF) as u8) as char;
    data[4] = (((m11 >> 0) & 0xFF) as u8) as char;
    data[5] = (((m11 >> 8) & 0xFF) as u8) as char;
    data[6] = (((m11 >> 16) & 0xFF) as u8) as char;
    data[7] = (((m11 >> 24) & 0xFF) as u8) as char;
    data[8] = (((m21 >> 0) & 0xFF) as u8) as char;
    data[9] = (((m21 >> 8) & 0xFF) as u8) as char;
    data[10] = (((m21 >> 16) & 0xFF) as u8) as char;
    data[11] = (((m21 >> 24) & 0xFF) as u8) as char;
    data[12] = (((m31 >> 0) & 0xFF) as u8) as char;
    data[13] = (((m31 >> 8) & 0xFF) as u8) as char;
    data[14] = (((m31 >> 16) & 0xFF) as u8) as char;
    data[15] = (((m31 >> 24) & 0xFF) as u8) as char;
    data[16] = (((m02 >> 0) & 0xFF) as u8) as char;
    data[17] = (((m02 >> 8) & 0xFF) as u8) as char;
    data[18] = (((m02 >> 16) & 0xFF) as u8) as char;
    data[19] = (((m02 >> 24) & 0xFF) as u8) as char;
    data[20] = (((m12 >> 0) & 0xFF) as u8) as char;
    data[21] = (((m12 >> 8) & 0xFF) as u8) as char;
    data[22] = (((m12 >> 16) & 0xFF) as u8) as char;
    data[23] = (((m12 >> 24) & 0xFF) as u8) as char;
    data[24] = (((m22 >> 0) & 0xFF) as u8) as char;
    data[25] = (((m22 >> 8) & 0xFF) as u8) as char;
    data[26] = (((m22 >> 16) & 0xFF) as u8) as char;
    data[27] = (((m22 >> 24) & 0xFF) as u8) as char;
    data[28] = (((m32 >> 0) & 0xFF) as u8) as char;
    data[29] = (((m32 >> 8) & 0xFF) as u8) as char;
    data[30] = (((m32 >> 16) & 0xFF) as u8) as char;
    data[31] = (((m32 >> 24) & 0xFF) as u8) as char;
    data[32] = (((m03 >> 0) & 0xFF) as u8) as char;
    data[33] = (((m03 >> 8) & 0xFF) as u8) as char;
    data[34] = (((m03 >> 16) & 0xFF) as u8) as char;
    data[35] = (((m13 >> 0) & 0xFF) as u8) as char;
    data[36] = (((m13 >> 8) & 0xFF) as u8) as char;
    data[37] = (((m13 >> 16) & 0xFF) as u8) as char;
    data[38] = (((m13 >> 24) & 0xFF) as u8) as char;
    data[39] = (((m23 >> 0) & 0xFF) as u8) as char;
    data[40] = (((m23 >> 8) & 0xFF) as u8) as char;
    data[41] = (((m23 >> 16) & 0xFF) as u8) as char;
    data[42] = (((m23 >> 24) & 0xFF) as u8) as char;
    data[43] = (((m33 >> 0) & 0xFF) as u8) as char;
    data[44] = (((m33 >> 8) & 0xFF) as u8) as char;
    data[45] = (((m33 >> 16) & 0xFF) as u8) as char;
    data[46] = (((m33 >> 24) & 0xFF) as u8) as char;
    data[47] = (((m33 >> 24) & 0xFF) as u8) as char;

    fix_zeroterminated_string(&mut data);

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
