enum InstrType {
    RType,
    IType,
    LType,
    SType,
    JType,
    BType,
}

fn get_opcode(instr: &str) -> u16 {
    match instr {
        "ADD"   => 0x0,
        "SUB"   => 0x1,
        "MUL"   => 0x2,
        "DIV"   => 0x3,
        "OR"    => 0x4,
        "AND"   => 0x5,
        "XOR"   => 0x6,
        "SLL"   => 0x7,
        "SRL"   => 0x8,
        "ADDI"  => 0x9,
        "LOAD"  => 0xA,
        "STORE" => 0xB,
        "JAL"   => 0xC,
        "JRL"   => 0xD,
        "BEQ"   => 0xE,
        "BLT"   => 0xF,
        _       => panic!("[ERROR] UNKNOWN OPCODE"),
    }
}

fn get_instr_type(opcode: u16) -> InstrType {
    match opcode {
        0x0..=0x8   => InstrType::RType,
        0x9         => InstrType::IType,
        0xA         => InstrType::LType,
        0xB         => InstrType::SType,
        0xC..=0xD   => InstrType::JType,
        0xE..=0xF   => InstrType::BType,
        _           => panic!("[ERROR] UNKNOWN INSTR TYPE"),
    }
}

fn get_register_addr(regname: &str) -> u16 {
    match regname {
        "ZR"    => 0x0,
        "B1"    => 0x1,
        "B2"    => 0x2,
        "B3"    => 0x3,
        "G0"    => 0x4,
        "G1"    => 0x5,
        "G2"    => 0x6,
        "G3"    => 0x7,
        "G4"    => 0x8,
        "G5"    => 0x9,
        "G6"    => 0xA,
        "G7"    => 0xB,
        "G8"    => 0xC,
        "G9"    => 0xD,
        "G10"   => 0xE,
        "G11"   => 0xF,
        _       => panic!("[ERROR] UNKNOWN REGISTER NAME"),
    }
}

fn get_imm_data(imm: &str) -> i16 {
    imm.parse().unwrap() 
}

fn main() {
    let code = [
        "ADDI 10 B1",
        "ADD B1 B2 B2",
        "ADDI -1 B1",
        "BLT B1 ZR -4",
        "JRL 0 ZR G11"
    ];

    let mut binary: u16;

    for instr in code {
        let _parsed_instr = instr.replace(",", " ");
        let parsed_instr: Vec<&str> = _parsed_instr.split(' ').collect();

        let opcode = get_opcode(parsed_instr[0]);
        let instr_type = get_instr_type(opcode);
        match instr_type {
            InstrType::RType    => {
                if parsed_instr.len() != 4  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let rs2 = get_register_addr(parsed_instr[1]);
                let rs1 = get_register_addr(parsed_instr[2]);
                let rd  = get_register_addr(parsed_instr[3]);
                binary = (rs2 << 12) | (rs1 << 8) | (rd << 4) | opcode;
            },
            InstrType::IType    => {
                if parsed_instr.len() != 3  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let imm = get_imm_data(parsed_instr[1]);
                let rd  = get_register_addr(parsed_instr[2]);
                binary = ((imm as u16) << 8) | (rd << 4) | opcode;
            },
            InstrType::LType    => {
                if parsed_instr.len() != 4  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let imm = get_imm_data(parsed_instr[1]);
                let rs1 = get_register_addr(parsed_instr[2]);
                let rd  = get_register_addr(parsed_instr[3]);
                binary = ((imm as u16) << 12) | (rs1 << 8) | (rd << 4) | opcode;
            },
            InstrType::SType    => {
                if parsed_instr.len() != 4  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let rs2 = get_register_addr(parsed_instr[1]);
                let rs1 = get_register_addr(parsed_instr[2]);
                let imm = get_imm_data(parsed_instr[3]);
                binary = (rs2 << 12) | (rs1 << 8) | (((0xF & imm) as u16) << 4) | opcode;
            },
            InstrType::JType    => {
                if parsed_instr.len() != 4  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let imm = get_imm_data(parsed_instr[1]);
                let rs1 = get_register_addr(parsed_instr[2]);
                let rd  = get_register_addr(parsed_instr[3]);
                binary = ((imm as u16) << 12) | (rs1 << 8) | (rd << 4) | opcode;
            },
            InstrType::BType    => {
                if parsed_instr.len() != 4  {
                    panic!("[ERROR] UNKNOWN SYNTAX");
                }
                let rs2 = get_register_addr(parsed_instr[1]);
                let rs1 = get_register_addr(parsed_instr[2]);
                let imm = get_imm_data(parsed_instr[3]);
                binary = (((0xFF & imm) as u16) << 8) | ((rs2 & 3) << 6) | ((rs1 & 3) << 4) | opcode;
            },
        }
        println!("{:04X}", binary);
    }
}
