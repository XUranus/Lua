mod binary;
mod vm;
mod api;
mod state;

use crate::api::{consts::*, LuaAPI};
use crate::state::LuaState;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;


/*×××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××*/
pub mod test {
    use super::state;
    use crate::api::{consts::*, LuaAPI};
    use crate::state::LuaState;

    use std::env;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    pub fn test1() {
        let mut ls = state::new_lua_state();

        ls.push_boolean(true);
        print_stack(&ls);
        ls.push_integer(10);
        print_stack(&ls);
        ls.push_nil();
        print_stack(&ls);
        ls.push_string("hello".to_string());
        print_stack(&ls);
        ls.push_value(-4);
        print_stack(&ls);
        ls.replace(3);
        print_stack(&ls);
        ls.set_top(6);
        print_stack(&ls);
        ls.remove(-3);
        print_stack(&ls);
        ls.set_top(-5);
        print_stack(&ls);
    }

    pub fn test2() {
        let mut ls = state::new_lua_state();

        ls.push_integer(1);
        ls.push_string("2.0".to_string());
        ls.push_string("3.0".to_string());
        ls.push_number(4.0);
        print_stack(&ls);

        ls.arith(LUA_OPADD);
        print_stack(&ls);
        ls.arith(LUA_OPBNOT);
        print_stack(&ls);
        ls.len(2);
        print_stack(&ls);
        ls.concat(3);
        print_stack(&ls);
        let x = ls.compare(1, 2, LUA_OPEQ);
        ls.push_boolean(x);
        print_stack(&ls);
    }


    fn print_stack(ls: &LuaState) {
        let top = ls.get_top();
        for i in 1..top + 1 {
            let t = ls.type_id(i);
            match t {
                LUA_TBOOLEAN => print!("[{}]", ls.to_boolean(i)),
                LUA_TNUMBER => print!("[{}]", ls.to_number(i)),
                LUA_TSTRING => print!("[{:?}]", ls.to_string(i)),
                _ => print!("[{}]", ls.type_name(t)), // other values
            }
        }
        println!();
    }

}



/**************************************************/
fn main() -> io::Result<()> {
    test::test2();

    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let proto = binary::undump(data);
        proto.list();
    }
    Ok(())
}