pub const END: u8 = 0x00;

pub const GET: u8 = 0x10;
pub const SET: u8 = 0x11;
pub const VAL: u8 = 0x12;

pub const ADD_LIT: u8 = 0x20;
pub const SUB_LIT: u8 = 0x21;
pub const SBF_LIT: u8 = 0x22;
pub const MUL_LIT: u8 = 0x23;
pub const AND_LIT: u8 = 0x24;
pub const ORB_LIT: u8 = 0x25;
pub const XOR_LIT: u8 = 0x26;

pub const ADD_MEM: u8 = 0x30;
pub const SUB_MEM: u8 = 0x31;
pub const SBF_MEM: u8 = 0x32;
pub const MUL_MEM: u8 = 0x33;
pub const AND_MEM: u8 = 0x34;
pub const ORB_MEM: u8 = 0x35;
pub const XOR_MEM: u8 = 0x36;

pub const INC: u8 = 0x40;
pub const DEC: u8 = 0x41;
pub const NOT: u8 = 0x42;

pub const JMP: u8 = 0x50;
pub const JFL: u8 = 0x51;
pub const JNF: u8 = 0x52;
pub const JZE: u8 = 0x53;
pub const JNZ: u8 = 0x54;
