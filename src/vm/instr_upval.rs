use super::instructions::Instruction;
use crate::api::LuaVM;
use crate::api::consts::*;

// R(A) := UpValue[B][RK(C)]
pub fn get_tab_up(i: u32, vm: &mut LuaVM) {
    /*let (mut a, mut b, c) = i.abc();
    a += 1; b += 1;

    vm.push_global_table();
    vm.get_rk(c);
    vm.get_table(-2);
    vm.replace(a);
    vm.pop(1);

    println!("lua_upvalue_index = {}",lua_upvalue_index(b))
*/
    let (mut a,mut b,mut c) = i.abc();
    a += 1;
    b += 1;
    vm.get_rk(c);
    vm.get_table(lua_upvalue_index(b));
    vm.replace(a);
}


pub fn set_tab_up(i: u32,vm: &mut LuaVM) {
    let (mut a,b,c) = i.abc();
    a += 1;
    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(lua_upvalue_index(a));
}

pub fn get_upval(i: u32, vm: &mut LuaVM) {
    let (mut a,mut b,_) = i.abc();
    a += 1;
    b += 1;
    println!("get_upval {} {} {}",b,a,lua_upvalue_index(b));
    vm.copy(lua_upvalue_index(b),a)
}

pub fn set_upval(i: u32, vm: &mut LuaVM) {
    let (mut a,mut b,_) = i.abc();
    a += 1;
    b += 1;
    vm.copy(a,lua_upvalue_index(b))
}

fn lua_upvalue_index(i: isize) -> isize {
    LUA_REGISTRYINDEX  - i
}
