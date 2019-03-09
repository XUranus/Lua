use super::lua_value::LuaValue;

pub struct LuaStack {
    vec: Vec<LuaValue>,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        LuaStack {
            vec: Vec::with_capacity(size),
        }
    }

    pub fn top(&self) -> isize {
        self.vec.len() as isize
    }

    //check to guarantee has more than n element
    pub fn check(&mut self, n: usize) {
        self.vec.reserve(n);
    }

    pub fn push(&mut self, val: LuaValue) {
        self.vec.push(val);
    }

    pub fn pop(&mut self) -> LuaValue {
        self.vec.pop().unwrap()
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        let abs_idx = self.abs_index(idx);
        abs_idx > 0 && abs_idx <= self.top()
    }

    pub fn get(&self, idx: isize) -> LuaValue {
        if self.is_valid(idx) {
            let idx = self.abs_index(idx) as usize - 1;
            self.vec[idx].clone() // TODO::
        } else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        if self.is_valid(idx) {
            let idx = self.abs_index(idx) as usize - 1;
            self.vec[idx] = val;
        } else {
            panic!("invalid index!");
        }
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.vec.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}
