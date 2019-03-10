mod lua_stack;
mod lua_state;
mod closure;
mod lua_value;
mod arith_ops;
mod cmp_ops;
mod math;
mod lua_table;

use crate::binary::chunk::Prototype;
pub use self::lua_state::LuaState;

pub fn new_lua_state() -> LuaState {
    LuaState::new()
}
