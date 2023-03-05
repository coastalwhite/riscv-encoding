use self::rv32i::Rv32IInstruction;

macro_rules! def_instruction {
    (
        $name:ident,
        $variant:ident,
        $opcode:literal,
        {
            $(
            $preset_field_name:ident = $preset_field_value:literal
            ),* $(,)?
        },
        {
            $(
            $field_name:ident:
                $field_type:ty[$field_size:literal$(|$field_min_bit:literal)?]
                = $field_value:ident
                $([$to_instr_lambda:expr, $to_variant_lambda:expr])?
            ),* $(,)?
        }
        $(,
        {
            $(
            $property_name:ident: $property_type:ty = $property_lambda:expr
            ),* $(,)?
        }
        )?
        $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $(
                $field_name: $field_type,
            )*
        }

        impl std::convert::TryFrom<$crate::fixed_field::$variant> for $name {
            type Error = ();

            #[inline]
            fn try_from(variant: $crate::fixed_field::$variant) -> Result<Self, Self::Error> {
                $(
                    if variant.$preset_field_name() != $preset_field_value {
                        return Err(());
                    }
                )*
                $(
                $(
                    if ($to_instr_lambda)(variant.$field_value).is_none() {
                        return Err(());
                    }
                )?
                )*

                if variant.opcode != $opcode {
                    return Err(());
                }

                Ok(unsafe { Self::from_variant_unchecked(variant) })
            }
        }

        impl From<$name> for $crate::fixed_field::$variant {
            #[inline]
            fn from(
                #[allow(unused_variables)]
                instruction: $name
            ) -> Self {
                Self {
                    opcode: $opcode,
                    $(
                        $preset_field_name: $preset_field_value,
                    )*
                    $(
                        $field_value: {
                            let field_value = instruction.$field_name;
                            $(
                            let field_value = ($to_variant_lambda)(field_value);
                            )?
                            field_value
                        },
                    )*
                }
            }
        }

        impl TryFrom<u32> for $name {
            type Error = <Self as std::convert::TryFrom<$crate::fixed_field::$variant>>::Error;

            #[inline]
            fn try_from(enc: u32) -> Result<Self, Self::Error> {
                <$name>::try_from(<$crate::fixed_field::$variant>::from(enc))
            }
        }

        impl From<$name> for u32 {
            #[inline]
            fn from(instruction: $name) -> Self {
                <u32>::from(<$crate::fixed_field::$variant>::from(instruction))
            }
        }


        impl $name {
            pub const OPCODE: u8 = $opcode;

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

            #[inline]
            pub unsafe fn from_variant_unchecked(
                #[allow(unused_variables)]
                variant: $crate::fixed_field::$variant
            ) -> Self {
                Self {
                    $(
                        $field_name: {
                            let field_value = variant.$field_value();
                            $(
                            let field_value = ($to_instr_lambda)(field_value);
                            let field_value = unsafe { field_value.unwrap_unchecked() };
                            )?
                            field_value
                        },
                    )*
                }
            }

            $(
            #[inline]
            pub fn $field_name(&self) -> $field_type {
                self.$field_name
            }
            )*

            $(
            $(
            #[inline]
            pub fn $property_name(&self) -> $property_type {
                ($property_lambda)(self)
            }
            )*
            )?
        }
    };
}

macro_rules! decode_tree {
    (
        $value:expr, $wrapping_enum:ty,
        $($opcode:literal => $variant:ident => [
            $(
                $(
                ($($field_name:ident$([$field_upper:literal:$field_lower:literal])? == $field_value:literal
                ),* $(,)?) =>)? $instruction:ident
            ),+ $(,)?
        ]),+ $(,)?
    ) => {
        match $value & 0x7F {
            $(
            $opcode => {
                let variant = $crate::fixed_field::$variant::from($value);
                $(
                if
                    true
                    $($(&& {
                        let field_value = variant.$field_name;
                        $(
                        let field_value = (field_value >> $field_lower) & ((1 << ($field_upper - $field_lower)) - 1);
                        )?
                        field_value
                    } == $field_value)*)?
                {
                    Some(<$wrapping_enum>::$instruction(unsafe { <$instruction>::from_variant_unchecked(variant) }))
                } else )* {
                    None
                }
            },
            )+
            _ => None,
        }
        
    };
}


pub enum Instruction {
    Rv32I(Rv32IInstruction),
    // Rv64I(Rv64IInstruction),
    // ZiFenceI(ZiFenceIInstruction),
    // ZiCsr(ZiCsrInstruction),
    // Rv32M(Rv32MInstruction),
    // Rv64M(Rv64MInstruction),
    // Rv32A(Rv32AInstruction),
    // Rv64A(Rv64AInstruction),
    // Rv32F(Rv32FInstruction),
    // Rv64F(Rv64FInstruction),
    // Rv32D(Rv32DInstruction),
    // Rv64D(Rv64DInstruction),
}

pub mod fixed_field;

pub mod rv32i;
