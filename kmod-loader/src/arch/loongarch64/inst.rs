#![allow(non_camel_case_types)]
use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct reg0i15_format {
    #[bits(15)]
    pub immediate: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct reg0i26_format {
    #[bits(10)]
    pub immediate_h: u32,
    #[bits(16)]
    pub immediate_l: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg1i20_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(20)]
    pub immediate: u32,
    #[bits(7)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg1i21_format {
    #[bits(5)]
    pub immediate_h: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(16)]
    pub immediate_l: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(22)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i5_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    pub immediate: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i6_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(6)]
    pub immediate: u32,
    #[bits(16)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i12_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(12)]
    pub immediate: u32,
    #[bits(10)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i14_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(14)]
    pub immediate: u32,
    #[bits(8)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2i16_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(16)]
    pub immediate: u32,
    #[bits(6)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg2bstrd_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(6)]
    lsbd: u32,
    #[bits(6)]
    msbd: u32,
    #[bits(10)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg3_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    rk: u32,
    #[bits(17)]
    pub opcode: u32,
}

#[bitfield(u32)]
pub struct reg3sa2_format {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rj: u32,
    #[bits(5)]
    rk: u32,
    #[bits(2)]
    pub immediate: u32,
    #[bits(15)]
    pub opcode: u32,
}
