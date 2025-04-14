use crate::{pic::Thunkable, RegistersX64};
use std::mem;

use super::Register;

#[repr(packed)]
struct CallAbs {
  // call [rip+8]
  opcode0: u8,
  opcode1: u8,
  dummy0: u32,
  // jmp +10
  dummy1: u8,
  dummy2: u8,
  // destination
  address: usize,
}

pub fn call_abs(destination: usize) -> Box<dyn Thunkable> {
  let code = CallAbs {
    opcode0: 0xFF,
    opcode1: 0x15,
    dummy0: 0x0_0000_0002,
    dummy1: 0xEB,
    dummy2: 0x08,
    address: destination,
  };

  let slice: [u8; 16] = unsafe { mem::transmute(code) };
  Box::new(slice.to_vec())
}

#[repr(packed)]
struct JumpAbs {
  // jmp +6
  opcode0: u8,
  opcode1: u8,
  dummy0: u32,
  // destination
  address: usize,
}

pub fn jmp_abs(destination: usize) -> Box<dyn Thunkable> {
  let code = JumpAbs {
    opcode0: 0xFF,
    opcode1: 0x25,
    dummy0: 0x0_0000_0000,
    address: destination,
  };

  let slice: [u8; 14] = unsafe { mem::transmute(code) };
  Box::new(slice.to_vec())
}

#[repr(packed)]
struct JccAbs {
  // jxx + 16
  opcode: u8,
  dummy0: u8,
  dummy1: u8,
  dummy2: u8,
  dummy3: u32,
  // destination
  address: usize,
}

pub fn jcc_abs(destination: usize, condition: u8) -> Box<dyn Thunkable> {
  let code = JccAbs {
    // Invert the condition in x64 mode to simplify the conditional jump logic
    opcode: 0x71 ^ condition,
    dummy0: 0x0E,
    dummy1: 0xFF,
    dummy2: 0x25,
    dummy3: 0x0000_0000,
    address: destination,
  };

  let slice: [u8; 16] = unsafe { mem::transmute(code) };
  Box::new(slice.to_vec())
}

pub fn mov_reg_extended(src: Register, dst: Register) -> [u8; 3] {
  let rex = 0x48;
  let opcode = 0x89;
  let src = src as u8;
  let dst = dst as u8;

  let m = 0b11 << 6;
  let src = src << 3;
  [rex, opcode, m | src | dst]
}

pub fn and_reg_i32_extended(register: Register, imm: i32) -> [u8; 7] {
  let rex = 0x48;
  let opcode = 0x81;
  let register = register as u8;
  let m = 0b11 << 6;
  let reg = 0b100 << 3;
  let mod_r_m = m | reg | register;
  let imm = imm.to_le_bytes();

  [rex, opcode, mod_r_m, imm[0], imm[1], imm[2], imm[3]]
}

pub const PUSH_ALL_REGS: &'static [u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/push_all_regs_x64.bin"));
pub const POP_ALL_REGS: &'static [u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/pop_all_regs_x64.bin"));

pub(crate) fn regs_to_arg0() -> Vec<u8> {
  use iced_x86::code_asm::*;
  let mut builder = CodeAssembler::new(64).unwrap();
  builder.mov(rdi, rsp).unwrap();
  // builder
  //  .add(rdi, core::mem::size_of::<RegistersX64>() as i32)
  //  .unwrap();
  builder.assemble(0x00).unwrap()
}
