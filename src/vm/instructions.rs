use super::opcodes::OPCODES;
use super::opcodes::*;
use crate::api::LuaVM;

use super::instr_for::*;
use super::instr_load::*;
use super::instr_misc::*;
use super::instr_ops::*;

const MAXARG_BX: isize = (1 << 18) - 1; // 262143
const MAXARG_SBX: isize = MAXARG_BX >> 1; // 131071

/*
 31       22       13       5    0
  +-------+^------+-^-----+-^---------
  |b=9bits |c=9bits |a=8bits|op=6bits|  iABC
  +-------+^------+-^-----+-^---------
  |    bx=18bits    |a=8bits|op=6bits|  iABx
  +-------+^------+-^-----+-^---------
  |   sbx=18bits    |a=8bits|op=6bits|  iAsBx
  +-------+^------+-^-----+-^---------
  |    ax=26bits         |  op=6bits |  iAx
  +-------+^------+-^-----+-^---------
 31      23      15       7      0
*/

pub trait Instruction {
    fn opcode(self) -> u8;
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn abc(self) -> (isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
    fn execute(self, vm: &mut LuaVM);
}

impl Instruction for u32 {
    fn opcode(self) -> u8 {
        self as u8 & 0x3F
    }

    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name
    }

    fn opmode(self) -> u8 {
        OPCODES[self.opcode() as usize].opmode
    }

    fn b_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].bmode
    }

    fn c_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].cmode
    }

    //fetch parameter from iABC mode
    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    //fetch parameter from iABX mode
    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    //fetch parameter from iAsBx mode
    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_SBX)
    }

    //fetch parameter from iAx
    fn ax(self) -> isize {
        (self >> 6) as isize
    }

    fn execute(self, vm: &mut LuaVM) {
        match self.opcode() {
            OP_MOVE => _move(self, vm),
            OP_LOADK => load_k(self, vm),
            OP_LOADKX => load_kx(self, vm),
            OP_LOADBOOL => load_bool(self, vm),
            OP_LOADNIL => load_nil(self, vm),
            // OP_GETUPVAL => (),
            // OP_GETTABUP => (),
            // OP_GETTABLE => (),
            // OP_SETTABUP => (),
            // OP_SETUPVAL => (),
            // OP_SETTABLE => (),
            // OP_NEWTABLE => (),
            // OP_SELF => (),
            OP_ADD => add(self, vm),
            OP_SUB => sub(self, vm),
            OP_MUL => mul(self, vm),
            OP_MOD => _mod(self, vm),
            OP_POW => pow(self, vm),
            OP_DIV => div(self, vm),
            OP_IDIV => idiv(self, vm),
            OP_BAND => band(self, vm),
            OP_BOR => bor(self, vm),
            OP_BXOR => bxor(self, vm),
            OP_SHL => shl(self, vm),
            OP_SHR => shr(self, vm),
            OP_UNM => unm(self, vm),
            OP_BNOT => bnot(self, vm),
            OP_NOT => not(self, vm),
            OP_LEN => length(self, vm),
            OP_CONCAT => concat(self, vm),
            OP_JMP => jmp(self, vm),
            OP_EQ => eq(self, vm),
            OP_LT => lt(self, vm),
            OP_LE => le(self, vm),
            OP_TEST => test(self, vm),
            OP_TESTSET => test_set(self, vm),
            // OP_CALL => (),
            // OP_TAILCALL => (),
            // OP_RETURN => (),
            OP_FORLOOP => for_loop(self, vm),
            OP_FORPREP => for_prep(self, vm),
            // OP_TFORCALL => (),
            // OP_TFORLOOP => (),
            // OP_SETLIST => (),
            // OP_CLOSURE => (),
            // OP_VARARG => (),
            // OP_EXTRAARG => (),
            _ => {
                println!("TODO::not implemnted op: {}\n",self.opname());
                //unimplemented!()
            }
        }
    }
}

//instruction print assist method
impl Instruction {
    pub fn print_operands(i: u32) {
        match i.opmode() {
            OP_MODE_ABC => Instruction::print_abc(i),
            OP_MODE_ABX => Instruction::print_abx(i),
            OP_MODE_ASBX => Instruction::print_asbx(i),
            OP_MODE_AX => Instruction::print_ax(i),
            _ => panic!("corrupt!"),
        }
    }

    fn print_abc(i: u32) {
        let (a, b, c) = i.abc();
        print!("{}", a);
        if i.b_mode() != OP_ARG_N {
            if b > 0xFF {
                print!(" {}", -1 - (b & 0xFF))
            } else {
                print!(" {}", b)
            }
        }
        if i.c_mode() != OP_ARG_N {
            if c > 0xFF {
                print!(" {}", -1 - (c & 0xFF))
            } else {
                print!(" {}", c)
            }
        }
    }

    fn print_abx(i: u32) {
        let (a, bx) = i.a_bx();
        print!("{}", a);
        if i.b_mode() == OP_ARG_K {
            print!(" {}", -1 - bx)
        } else if i.b_mode() == OP_ARG_U {
            print!(" {}", bx)
        }
    }

    fn print_asbx(i: u32) {
        let (a, sbx) = i.a_sbx();
        print!("{} {}", a, sbx);
    }

    fn print_ax(i: u32) {
        let ax = i.ax();
        print!("{}", -1 - ax);
    }
}