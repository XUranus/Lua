mod api;
mod binary;
mod state;
mod vm;

use crate::api::{consts::*, LuaAPI, LuaVM};
use crate::binary::chunk::Prototype;
use crate::state::LuaState;
use crate::vm::instructions::Instruction;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let proto = binary::undump(data);
        lua_main(proto);
    }
    Ok(())
}

fn lua_main(proto: Prototype) {
    let nregs = proto.max_stack_size;
    let mut ls = state::new_lua_state((nregs + 8) as usize, proto);
    ls.set_top(nregs as isize);
    loop {
        let pc = ls.pc();
        let instr = ls.fetch();
        if instr.opcode() != vm::opcodes::OP_RETURN {
            instr.execute(&mut ls);
            print!("[{:04}] {} ", pc + 1, instr.opname());
            ls.print_stack();
        } else {
            break;
        }
    }
}
