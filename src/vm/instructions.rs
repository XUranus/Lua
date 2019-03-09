use super::opcodes::OPCODES;
use super::opcodes::*;

const MAXARG_BX: isize = (1 << 18) - 1; // 262143
const MAXARG_SBX: isize = MAXARG_BX >> 1; // 131071

/*
 31       22       13       5    0
  +-------+^------+-^-----+-^---------
  |b=9bits |c=9bits |a=8bits|op=6bits|
  +-------+^------+-^-----+-^---------
  |    bx=18bits    |a=8bits|op=6bits|
  +-------+^------+-^-----+-^---------
  |   sbx=18bits    |a=8bits|op=6bits|
  +-------+^------+-^-----+-^---------
  |    ax=26bits         |  op=6bits |
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