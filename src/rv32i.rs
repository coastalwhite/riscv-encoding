def_instruction! {
    Lui, UType, 0b011_0111,
    {},
    {
        rd: u8[5] = rd,
        imm: u32[20] = imm,
    }
}

def_instruction! {
    AuiPc, UType, 0b001_0111,
    {},
    {
        rd: u8[5] = rd,
        imm: u32[20] = imm,
    }
}

def_instruction! {
    Jal, JType, 0b110_1111,
    {},
    {
        rd: u8[5] = rd,
        imm: u32[20] = imm,
    }
}

def_instruction! {
    JalR, IType, 0b110_0111,
    {
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[12] = imm,
    }
}

def_instruction! {
    Beq, BType, 0b110_0011,
    {
        funct3 = 0b000,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Bne, BType, 0b110_0011,
    {
        funct3 = 0b001,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Blt, BType, 0b110_0011,
    {
        funct3 = 0b100,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Bge, BType, 0b110_0011,
    {
        funct3 = 0b101,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Bltu, BType, 0b110_0011,
    {
        funct3 = 0b110,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Bgeu, BType, 0b110_0011,
    {
        funct3 = 0b111,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Lb, IType, 0b000_0011,
    {
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Lh, IType, 0b000_0011,
    {
        funct3 = 0b001,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Lw, IType, 0b000_0011,
    {
        funct3 = 0b010,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Lbu, IType, 0b000_0011,
    {
        funct3 = 0b100,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Lhu, IType, 0b000_0011,
    {
        funct3 = 0b101,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Sb, SType, 0b100_0011,
    {
        funct3 = 0b000,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Sh, SType, 0b100_0011,
    {
        funct3 = 0b001,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Sw, SType, 0b100_0011,
    {
        funct3 = 0b010,
    },
    {
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Addi, IType, 0b001_0011,
    {
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Slti, IType, 0b001_0011,
    {
        funct3 = 0b010,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Sltiu, IType, 0b001_0011,
    {
        funct3 = 0b011,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Xori, IType, 0b001_0011,
    {
        funct3 = 0b100,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Ori, IType, 0b001_0011,
    {
        funct3 = 0b110,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Andi, IType, 0b001_0011,
    {
        funct3 = 0b111,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        imm: u16[13|1] = imm,
    }
}

def_instruction! {
    Slli, IType, 0b001_0011,
    {
        funct3 = 0b001,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        shamt: u8[5] = imm[
            |imm: u16| {
                if imm & 0xFE != 0x000 {
                    return None;
                }

                Some((imm & 0x1F) as u8)
            },
            |shamt: u8| {
                (shamt as u16) | 0x000
            }
        ],
    }
}

def_instruction! {
    Srli, IType, 0b001_0011,
    {
        funct3 = 0b101,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        shamt: u8[5] = imm[
            |imm: u16| {
                if imm & 0xFE0 != 0x000 {
                    return None;
                }

                Some((imm & 0x1F0) as u8)
            },
            |shamt: u8| {
                (shamt as u16) | 0x000
            }
        ],
    }
}

def_instruction! {
    Srai, IType, 0b001_0011,
    {
        funct3 = 0b101,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        shamt: u8[5] = imm[
            |imm: u16| {
                if imm & 0xFE0 != 0x400 {
                    return None;
                }

                Some((imm & 0x1F) as u8)
            },
            |shamt: u8| {
                (shamt as u16) | 0x400
            }
        ],
    }
}

def_instruction! {
    Add, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Sub, RType, 0b011_0011,
    {
        funct7 = 0b010_0000,
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Sll, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b001,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Slt, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b010,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Sltu, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b011,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Xor, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b100,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Srl, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b101,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Sra, RType, 0b011_0011,
    {
        funct7 = 0b010_0000,
        funct3 = 0b101,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Or, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b110,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    And, RType, 0b011_0011,
    {
        funct7 = 0b000_0000,
        funct3 = 0b111,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        rs2: u8[5] = rs2,
    }
}

def_instruction! {
    Fence, IType, 0b000_1111,
    {
        funct3 = 0b000,
    },
    {
        rd: u8[5] = rd,
        rs1: u8[5] = rs1,
        fm_pred_succ: u16[12] = imm,
    },
    {
        fm: u8 = |i: &Fence| { ((i.fm_pred_succ() & 0xF00) >> 8) as u8 },
        pred: u8 = |i: &Fence| { ((i.fm_pred_succ() & 0x0F0) >> 4) as u8 },
        succ: u8 = |i: &Fence| { ((i.fm_pred_succ() & 0x00F) >> 0) as u8 },
    }
}

def_instruction! {
    FenceTso, IType, 0b000_1111,
    {
        rd = 0b0_0000,
        funct3 = 0b000,
        rs1 = 0b0_0000,
        imm = 0b1000_0011_0011,
    },
    {}
}

def_instruction! {
    Pause, IType, 0b000_1111,
    {
        rd = 0b0_0000,
        funct3 = 0b000,
        rs1 = 0b0_0000,
        imm = 0b0000_0000_0000,
    },
    {}
}

def_instruction! {
    ECall, IType, 0b111_0011,
    {
        rd = 0b0_0000,
        funct3 = 0b000,
        rs1 = 0b0_0000,
        imm = 0b0000_0000_0000,
    },
    {}
}

def_instruction! {
    EBreak, IType, 0b111_0011,
    {
        rd = 0b0_0000,
        funct3 = 0b000,
        rs1 = 0b0_0000,
        imm = 0b0000_0000_0001,
    },
    {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rv32IInstruction {
    Lui(Lui),
    AuiPc(AuiPc),
    Jal(Jal),
    JalR(JalR),
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Bge(Bge),
    Bltu(Bltu),
    Bgeu(Bgeu),
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Lbu(Lbu),
    Lhu(Lhu),
    Sb(Sb),
    Sh(Sh),
    Sw(Sw),
    Addi(Addi),
    Slti(Slti),
    Sltiu(Sltiu),
    Xori(Xori),
    Ori(Ori),
    Andi(Andi),
    Slli(Slli),
    Srli(Srli),
    Srai(Srai),
    Add(Add),
    Sub(Sub),
    Sll(Sll),
    Slt(Slt),
    Sltu(Sltu),
    Xor(Xor),
    Srl(Srl),
    Sra(Sra),
    Or(Or),
    And(And),
    Fence(Fence),
    FenceTso(FenceTso),
    Pause(Pause),
    ECall(ECall),
    EBreak(EBreak),
}

impl TryFrom<u32> for Rv32IInstruction {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        decode_tree! {
            value, Rv32IInstruction,
            0b011_0111 => UType => [ Lui ],
            0b001_0111 => UType => [ AuiPc ],
            0b110_1111 => JType => [ Jal ],
            0b110_0111 => IType => [ JalR ],
            0b110_0011 => BType => [
                ( funct3 == 0b000 ) => Beq,
                ( funct3 == 0b001 ) => Bne,
                ( funct3 == 0b100 ) => Blt,
                ( funct3 == 0b101 ) => Bge,
                ( funct3 == 0b110 ) => Bltu,
                ( funct3 == 0b111 ) => Bgeu,
            ],
            0b000_0011 => IType => [
                ( funct3 == 0b000 ) => Lb,
                ( funct3 == 0b001 ) => Lh,
                ( funct3 == 0b010 ) => Lw,
                ( funct3 == 0b100 ) => Lbu,
                ( funct3 == 0b101 ) => Lhu,
            ],
            0b010_0011 => SType => [
                ( funct3 == 0b000 ) => Sb,
                ( funct3 == 0b001 ) => Sh,
                ( funct3 == 0b010 ) => Sw,
            ],
            0b001_0011 => IType => [
                ( funct3 == 0b000 ) => Addi,
                ( funct3 == 0b010 ) => Slti,
                ( funct3 == 0b011 ) => Sltiu,
                ( funct3 == 0b100 ) => Xori,
                ( funct3 == 0b110 ) => Ori,
                ( funct3 == 0b111 ) => Andi,
                ( funct3 == 0b001, imm[12:5] == 0b000_0000 ) => Slli,
                ( funct3 == 0b101, imm[12:5] == 0b000_0000 ) => Srli,
                ( funct3 == 0b101, imm[12:5] == 0b010_0000 ) => Srai,
            ],
            0b011_0011 => RType => [
                ( funct3 == 0b000, funct7 == 0b000_0000 ) => Add,
                ( funct3 == 0b000, funct7 == 0b010_0000 ) => Sub,
                ( funct3 == 0b001, funct7 == 0b000_0000 ) => Sll,
                ( funct3 == 0b010, funct7 == 0b000_0000 ) => Slt,
                ( funct3 == 0b011, funct7 == 0b000_0000 ) => Sltu,
                ( funct3 == 0b100, funct7 == 0b000_0000 ) => Xor,
                ( funct3 == 0b101, funct7 == 0b000_0000 ) => Srl,
                ( funct3 == 0b101, funct7 == 0b010_0000 ) => Sra,
                ( funct3 == 0b110, funct7 == 0b000_0000 ) => Or,
                ( funct3 == 0b111, funct7 == 0b000_0000 ) => And,
            ],
            0b000_1111 => IType => [
                (
                    rd == 0b0_0000,
                    funct3 == 0b000,
                    rs1 == 0b0_0000,
                    imm == 0b1000_0011_0011
                ) => FenceTso,
                (
                    rd == 0b0_0000,
                    funct3 == 0b000,
                    rs1 == 0b0_0000,
                    imm == 0b0000_0001_0000
                ) => Pause,
                ( funct3 == 0b000 ) => Fence,
            ],
            0b111_0011 => IType => [
                (
                    rd == 0b0_0000,
                    funct3 == 0b000,
                    rs1 == 0b0_0000,
                    imm == 0b0000_0000_0000
                ) => ECall,
                (
                    rd == 0b0_0000,
                    funct3 == 0b000,
                    rs1 == 0b0_0000,
                    imm == 0b0000_0000_0001
                ) => EBreak,
            ],
        }
        .ok_or(())
    }
}

#[test]
fn decode() {
    let instructions: &[u32] = &[
        0x0000_0313, //     li	t1,0
        0x0000_0393, //     li	t2,0
        0x0000_0e13, //     li	t3,0
        0x064e_0e13, //     addi	t3,t3,100
        // <begin>:
        0x01c3_5863, //     bge	t1,t3,20 <end>
        0x0073_0333, //     add	t1,t1,t2
        0x0013_0313, //     addi	t1,t1,1
        0xff5f_f06f, //     j	10 <begin>
        // <end>:
        0x0000_0313, //     li	t1,0
    ];

    println!(
        "{:?}",
        instructions
            .iter()
            .map(|i| Rv32IInstruction::try_from(*i))
            .collect::<Vec<Result<Rv32IInstruction, ()>>>()
    );
    assert!(false);
}
