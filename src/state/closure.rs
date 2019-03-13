use crate::binary::chunk::Prototype;
use crate::api::RustFn;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::lua_value::LuaValue;

pub struct Closure {
    pub proto: Rc<Prototype>,//lua closure
    pub rust_fn: Option<RustFn>,//rust closure
    pub upvalues: RefCell<Vec<RefCell<LuaValue>>>,
    rdm: usize,
}

//TODO::?usage?
impl Hash for Closure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}

impl Closure {

    pub fn new_fake_closure() -> Closure {
        Closure {
            proto: new_empty_prototype(), // TODO
            rust_fn: None,
            rdm: super::math::random(),
            upvalues: RefCell::new(Vec::new()) //empty
        }
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Closure {
        let len = proto.upvalues.len();
        let mut vec = Vec::new();
        for i in 0..len {
            vec.push(RefCell::new(LuaValue::Nil));
        }
        Closure {
            upvalues: RefCell::new(vec),
            proto,
            rust_fn: None,
            rdm: super::math::random()
        }
    }

    pub fn new_rust_closure(f: RustFn,n_upvals: usize) -> Closure {
        let len = n_upvals;
        let mut vec = Vec::new();
        for i in 0..len {
            vec.push(RefCell::new(LuaValue::Nil));
        }
        Closure {
            proto: new_empty_prototype(), // TODO
            rust_fn: Some(f),
            rdm: super::math::random(),
            upvalues:RefCell::new(vec)
        }
    }

    pub fn is_fake(&self) -> bool {
        return match (self.proto.is_empty(),self.rust_fn) {
            (true,None) => true,
            _ => false
        }
    }
}

fn new_empty_prototype() -> Rc<Prototype> {
    Rc::new(Prototype {
        source: None, // debug
        line_defined: 0,
        last_line_defined: 0,
        num_params: 0,
        is_vararg: 0,
        max_stack_size: 0,
        code: vec![],
        constants: vec![],
        upvalues: vec![],
        protos: vec![],
        line_info: vec![],     // debug
        loc_vars: vec![],      // debug
        upvalue_names: vec![], // debug
    })
}
