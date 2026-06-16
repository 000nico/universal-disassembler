use capstone::prelude::*;
use crate::tools::Instruction;

pub fn disassemble_bytes(aob: &[u8]) -> Vec<Instruction> {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .unwrap();
    let instructions = cs.disasm_all(aob, 0x0);
    let disasm = match instructions {
        Ok(insns) => insns,
        Err(_) => return Vec::new(),
    };

    disasm
        .iter()
        .map(|i| Instruction {
            offset: i.address(),
            bytes: i.bytes().to_vec(),
            mnemonic: i.mnemonic().unwrap_or("").to_string(),
            operands: i.op_str().unwrap_or("").to_string(),
        })
        .collect()
}