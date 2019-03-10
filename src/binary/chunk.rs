use std::rc::Rc;
use crate::vm::instructions::Instruction;

#[allow(dead_code)]
struct BinaryChunk {
    header: Header,
    size_upvalues: u8,
    main_func: Prototype,
}


/*
* Lua header is used to check version,data size and endian.
* if not equal,will directly panic
*/
#[allow(dead_code)]
struct Header {
    signature: [u8; 4], //magic_number "x1bLua" 0x1b4c7561
    version: u8, //must be equal to local version,version = major_version*16 + minor_version
    format: u8, //officially be 0x00
    luac_data: [u8; 6], //0x1993 lua be released,0x0D0A
    c_int_size: u8, //0x04
    c_size_t_size: u8, //0x08
    instruction_size: u8, //0x04
    lua_integer_size: u8, //0x08
    lua_number_size: u8, //0x08
    luac_int: i64, //0x5678 used to check big/little endian
    luac_num: f64, //check float point format,value = 370
}

// function prototype
pub struct Prototype {
    pub source: Option<String>,//only in main func has value,otherwise empty
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Rc<Prototype>>,//sub func
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
}

pub struct Upvalue {
    pub instack: u8,
    pub idx: u8,
}

pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}

/* header check constants */
pub const LUA_SIGNATURE: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61]; // "\x1bLua"
pub const LUAC_VERSION: u8 = 0x53;
pub const LUAC_FORMAT: u8 = 0;
pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]; // "\x19\x93\r\n\x1a\n"
pub const CINT_SIZE: u8 = 4;
pub const CSIZET_SIZE: u8 = 8;
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

/*constants tags*/
pub const TAG_NIL: u8 = 0x00;
pub const TAG_BOOLEAN: u8 = 0x01;
pub const TAG_NUMBER: u8 = 0x03;
pub const TAG_SHORT_STR: u8 = 0x04;
pub const TAG_LONG_STR: u8 = 0x14;
pub const TAG_INTEGER: u8 = 0x13;


impl Prototype{
    //$:luac -l [chunkname],print info
    pub fn list(&self) {
        self.print_header();
        self.print_code();
        self.print_detail();
        for p in &(self.protos) {
            p.list();
        }
    }

    // main <@hello_world.lua:0,0> (4 instructions at 0x0000000)
    fn print_header(&self) {
        let func_type = if self.line_defined > 0 { "function" } else { "main" };
        let vararg_flag = if self.is_vararg > 0 { "+" } else { "" };
        let source = self.source.as_ref().map(|x| x.as_str()).unwrap_or("");//TODO:：？

        print!("\n{}", func_type);
        print!(" <{}:{},{}>", source, self.line_defined, self.last_line_defined);
        print!(" ({} instructions)\n", self.code.len());
        print!("{}{} params", self.num_params, vararg_flag);
        print!(", {} slots", self.max_stack_size);
        print!(", {} upvalues", self.upvalues.len());
        print!(", {} locals", self.loc_vars.len());
        print!(", {} constants", self.constants.len());
        print!(", {} functions\n", self.protos.len());
    }

    fn print_code(&self) {
        for pc in 0..self.code.len() {
            let line = self.line_info.get(pc).map(|n| n.to_string()).unwrap_or(String::new());
            let ins = self.code[pc];
            print!("\t{}\t[{}]\t{} ", pc + 1, line,ins.opname());
            Instruction::print_operands(ins);
            println!();
        }
    }

    fn print_detail(&self) {
        self.print_consts();
        self.print_locals();
        self.print_upvals()
    }

    fn print_consts(&self) {
        let n = self.constants.len();
        println!("constants ({}):", n);
        for i in 0..n {
            Prototype::print_const(i + 1, &self.constants[i]);
        }
    }

    fn print_const(n: usize, k: &Constant) {
        match k {
            Constant::Nil => println!("\t{}\tnil", n),
            Constant::Boolean(b) => println!("\t{}\t{}", n, b),
            Constant::Number(x) => println!("\t{}\t{}", n, x),
            Constant::Integer(i) => println!("\t{}\t{}", n, i),
            Constant::Str(s) => println!("\t{}\t{:?}", n, s),
        }
    }

    fn print_locals(&self) {
        let n = self.loc_vars.len();
        println!("locals ({}):", n);
        for i in 0..n {
            let var = &self.loc_vars[i];
            println!("\t{}\t{}\t{}\t{}", i, var.var_name, var.start_pc + 1, var.end_pc + 1);
        }
    }

    fn print_upvals(&self) {
        let n = self.upvalues.len();
        println!("upvalues ({}):", n);
        for i in 0..n {
            let upval = &self.upvalues[i];
            let name = self.upvalue_names.get(i).map(|x| x.as_str()).unwrap_or("");
            println!("\t{}\t{}\t{}\t{}", i, name, upval.instack, upval.idx);
        }
    }
}