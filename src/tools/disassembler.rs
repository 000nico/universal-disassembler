extern crate capstone;
use capston::prelude::*;

pub fn dissassemble_bytes(aob: &[u8]) -> Vec<String> {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .unwrap();

    cs.disasm_all(bytes, 0x0)
        .unwrap()
        .iter()
        .map(|i| {
            format!(
                "{} {}",
                i.mnemonic().unwrap_or(""),
                i.op_str().unwrap_or("")
            )
        })
        .collect()
}