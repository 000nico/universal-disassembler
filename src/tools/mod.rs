pub mod patterns;
pub mod strings;
pub mod hashes;
pub mod entropy;
pub mod disassembler;
pub struct byte_changes {
    address: usize,
    new: u8,
}
pub struct Instruction {
    pub offset: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
}