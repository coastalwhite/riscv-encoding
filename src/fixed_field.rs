macro_rules! variant {
    ($variant:ident {
        $($field_name:ident: $field_type:ty[$field_size:literal$(|$field_min_bit:literal)?]),* $(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $variant {
            $(
            pub(crate) $field_name: $field_type,
            )*
        }

        impl $variant {
            $(
            #[inline]
            pub fn $field_name(self) -> $field_type {
                debug_assert!(u64::from(self.$field_name) < (1u64 << $field_size));
                $(debug_assert!(u64::from(self.$field_name) & ((1u64 << $field_min_bit) - 1) == 0);)?
                self.$field_name
            }
            )*

            #[inline]
            pub fn new($($field_name: $field_type),*) -> Option<Self> {
                if
                    false
                    $(
                        || dbg!(u64::from($field_name) >= (1u64 << $field_size))
                        $(|| dbg!(u64::from($field_name) & ((1u64 << $field_min_bit) - 1) != 0))?
                    )*
                {
                    return None;
                }

                Some(Self {
                    $(
                    $field_name,
                    )*
                })
            }
        }
    };
}

variant! {
    RType {
        funct7: u8[7],
        rs2: u8[5],
        rs1: u8[5],
        funct3: u8[3],
        rd: u8[5],
        opcode: u8[7],
    }
}

variant! {
    IType {
        imm: u16[12],
        rs1: u8[5],
        funct3: u8[3],
        rd: u8[5],
        opcode: u8[7],
    }
}

variant! {
    SType {
        imm: u16[12],
        rs2: u8[5],
        rs1: u8[5],
        funct3: u8[3],
        opcode: u8[7],
    }
}

variant! {
    BType {
        imm: u16[13|1],
        rs2: u8[5],
        rs1: u8[5],
        funct3: u8[3],
        opcode: u8[7],
    }
}

variant! {
    UType {
        imm: u32[32|12],
        rd: u8[5],
        opcode: u8[7],
    }
}

variant! {
    JType {
        imm: u32[21|1],
        rd: u8[5],
        opcode: u8[7],
    }
}

impl From<u32> for RType {
    fn from(enc: u32) -> Self {
        Self {
            funct7: get_u8_from_to(enc, 25, 31),
            rs2: get_u8_from_to(enc, 20, 24),
            rs1: get_u8_from_to(enc, 15, 19),
            funct3: get_u8_from_to(enc, 12, 14),
            rd: get_u8_from_to(enc, 7, 11),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<u32> for IType {
    fn from(enc: u32) -> Self {
        Self {
            imm: get_u16_from_to(enc, 20, 31),
            rs1: get_u8_from_to(enc, 15, 19),
            funct3: get_u8_from_to(enc, 12, 14),
            rd: get_u8_from_to(enc, 7, 11),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<u32> for SType {
    fn from(enc: u32) -> Self {
        let imm = (get_u16_from_to(enc, 25, 31) << 5) | get_u16_from_to(enc, 7, 11);

        Self {
            imm,
            rs2: get_u8_from_to(enc, 20, 24),
            rs1: get_u8_from_to(enc, 15, 19),
            funct3: get_u8_from_to(enc, 12, 14),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<u32> for BType {
    fn from(enc: u32) -> Self {
        let imm = (get_u16_from_to(enc, 31, 31) << 12)
            | (get_u16_from_to(enc, 7, 7) << 11)
            | (get_u16_from_to(enc, 25, 30) << 5)
            | (get_u16_from_to(enc, 8, 11) << 1);

        Self {
            imm,
            rs2: get_u8_from_to(enc, 20, 24),
            rs1: get_u8_from_to(enc, 15, 19),
            funct3: get_u8_from_to(enc, 12, 14),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<u32> for UType {
    fn from(enc: u32) -> Self {
        let imm = enc & 0xFFFF_F000;

        Self {
            imm,
            rd: get_u8_from_to(enc, 7, 11),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<u32> for JType {
    fn from(enc: u32) -> Self {
        let imm_20 = get_u16_from_to(enc, 31, 31) as u32;
        let imm_1_10 = get_u16_from_to(enc, 21, 30) as u32;
        let imm_11 = get_u16_from_to(enc, 20, 20) as u32;
        let imm_12_19 = get_u16_from_to(enc, 12, 19) as u32;
        let imm = (imm_20 << 20) | (imm_12_19 << 12) | (imm_11 << 11) | (imm_1_10 << 1);

        Self {
            imm,
            rd: get_u8_from_to(enc, 7, 11),
            opcode: get_u8_from_to(enc, 0, 6),
        }
    }
}

impl From<RType> for u32 {
    fn from(enc: RType) -> Self {
        let RType {
            funct7,
            rs2,
            rs1,
            funct3,
            rd,
            opcode,
        } = enc;
        (opcode as u32)
            | ((rd as u32) << 7)
            | ((funct3 as u32) << 12)
            | ((rs1 as u32) << 15)
            | ((rs2 as u32) << 20)
            | ((funct7 as u32) << 25)
    }
}

impl From<IType> for u32 {
    fn from(enc: IType) -> Self {
        let IType {
            imm,
            rs1,
            funct3,
            rd,
            opcode,
        } = enc;
        (opcode as u32)
            | ((rd as u32) << 7)
            | ((funct3 as u32) << 12)
            | ((rs1 as u32) << 15)
            | ((imm as u32) << 20)
    }
}

impl From<SType> for u32 {
    fn from(enc: SType) -> Self {
        let SType {
            imm,
            rs1,
            rs2,
            funct3,
            opcode,
        } = enc;
        let imm5_11 = (imm & 0xFE0) >> 5;
        let imm0_4 = imm & 0x1F;
        (opcode as u32)
            | ((imm0_4 as u32) << 7)
            | ((funct3 as u32) << 12)
            | ((rs1 as u32) << 15)
            | ((rs2 as u32) << 20)
            | ((imm5_11 as u32) << 25)
    }
}

impl From<BType> for u32 {
    fn from(enc: BType) -> Self {
        let BType {
            imm,
            rs1,
            rs2,
            funct3,
            opcode,
        } = enc;
        let imm12 = (imm & 0x1000) >> 12;
        let imm5_10 = (imm & 0x7E0) >> 5;
        let imm1_4 = (imm & 0x1E) >> 1;
        let imm11 = (imm & 0x0800) >> 11;
        (opcode as u32)
            | ((imm11 as u32) << 7)
            | ((imm1_4 as u32) << 8)
            | ((funct3 as u32) << 12)
            | ((rs1 as u32) << 15)
            | ((rs2 as u32) << 20)
            | ((imm5_10 as u32) << 25)
            | ((imm12 as u32) << 31)
    }
}

impl From<UType> for u32 {
    fn from(enc: UType) -> Self {
        let UType { imm, rd, opcode } = enc;
        let imm12_31 = imm >> 12;
        (opcode as u32) | ((rd as u32) << 7) | ((imm12_31 as u32) << 12)
    }
}

impl From<JType> for u32 {
    fn from(enc: JType) -> Self {
        let JType { imm, rd, opcode } = enc;
        let imm12_19 = (imm & 0x000F_F000) >> 12;
        let imm11 = (imm & 0x0000_0800) >> 11;
        let imm1_10 = (imm & 0x0000_07FE) >> 1;
        let imm20 = (imm & 0x0010_0000) >> 20;
        (opcode as u32)
            | ((rd as u32) << 7)
            | ((imm12_19 as u32) << 12)
            | ((imm11 as u32) << 20)
            | ((imm1_10 as u32) << 21)
            | ((imm20 as u32) << 31)
    }
}

#[inline]
const fn get_u8_from_to(enc: u32, from: usize, to: usize) -> u8 {
    debug_assert!(to - from < 8);
    get_u16_from_to(enc, from, to) as u8
}

#[inline]
const fn get_u16_from_to(enc: u32, from: usize, to: usize) -> u16 {
    debug_assert!(from < 32);
    debug_assert!(to < 32);
    debug_assert!(from <= to);
    debug_assert!(to - from < 16);

    let mask = (1 << (to - from + 1)) - 1;
    ((enc >> from) & mask) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encoding_inversion {
        use super::*;
        const MASKS: [u32; 4] = [0, 0xFFFF_FFFF, 0x0123_ABCD, 0xAAAA_AAAA];

        #[test]
        fn rtype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(RType::from(mask)));
            }
        }

        #[test]
        fn itype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(IType::from(mask)));
            }
        }

        #[test]
        fn stype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(SType::from(mask)));
            }
        }

        #[test]
        fn btype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(BType::from(mask)));
            }
        }

        #[test]
        fn utype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(UType::from(mask)));
            }
        }

        #[test]
        fn jtype() {
            for mask in MASKS {
                assert_eq!(mask, u32::from(JType::from(mask)));
            }
        }
    }

    mod creation {
        use super::*;

        #[test]
        fn rtype() {
            assert!(RType::new(0b0, 0b0, 0b0, 0b0, 0b0, 0b0100_0000).is_some());
            assert!(RType::new(0b0, 0b0, 0b0, 0b0, 0b0, 0b1000_0000).is_none());
        }

        #[test]
        fn itype() {}

        #[test]
        fn stype() {}

        #[test]
        fn btype() {
            assert!(BType::new(0b0, 0b0, 0b0, 0b0, 0b0100_0000).is_some());
            assert!(BType::new(0b1, 0b0, 0b0, 0b0, 0b0100_0000).is_none());
        }

        #[test]
        fn utype() {
            assert!(UType::new(0x1000, 0b0, 0b0100_0000).is_some());
            assert!(UType::new(0x1800, 0b0, 0b0100_0000).is_none());
        }

        #[test]
        fn jtype() {
            assert!(JType::new(0b0, 0b0, 0b0100_0000).is_some());
            assert!(JType::new(0b1, 0b0, 0b0100_0000).is_none());
        }
    }
}
