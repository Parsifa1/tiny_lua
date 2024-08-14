#![allow(dead_code)]
use crate::disasm::reader::Reader;
use core::fmt;

// binary_chunk 常量定义

pub const LUA_SIGNATURE: &[u8; 4] = b"\x1bLua";
pub const LUAC_DATA: &[u8; 6] = b"\x19\x93\r\n\x1a\n";
pub const LUAC_VERSION: u8 = 0x53;
pub const LUAC_FORMAT: u8 = 0;
pub const CINT_SIZE: u8 = 4;
pub const CSIZET_SIZE: u8 = 8;
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

/// 二进制文件
#[allow(dead_code)]
struct BinaryChunk {
    header: Header,
    size_upvalues: u8,
    main_func: Prototype,
}

/// 头部
pub struct Header {
    pub signature: [u8; 4],
    pub version: u8,
    pub format: u8,
    pub luac_data: [u8; 6],
    pub cint_size: u8,
    pub sizet_size: u8,
    pub instruction_size: u8,
    pub lua_integer_size: u8,
    pub lua_number_size: u8,
    pub luac_int: i64,
    pub luac_num: f64,
}

/// 函数原型
pub struct Prototype {
    pub source: String,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Prototype>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
}

/// 常量
pub enum Constant {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    Str(String),
}

/// Upvalue
pub struct Upvalue {
    pub instack: u8,
    pub idx: u8,
}

/// 局部变量
pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

impl Prototype {
    fn write_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut func_type = "main";
        //header
        if self.line_defined > 0 {
            func_type = "function";
        }
        let mut varagflag = "";
        if self.is_vararg > 0 {
            varagflag = "+";
        }
        writeln!(
            f,
            "\n{} <{}:{},{}> ({} instructions)",
            func_type,
            self.source,
            self.line_defined,
            self.last_line_defined,
            self.code.len()
        )?;
        write!(
            f,
            "{}{} params, {} slots, {} upvalues, ",
            self.num_params,
            varagflag,
            self.max_stack_size,
            self.upvalues.len()
        )?;
        writeln!(
            f,
            "{} locals, {} constants, {} functions",
            self.loc_vars.len(),
            self.constants.len(),
            self.protos.len()
        )?;
        Ok(())
    }

    fn write_code(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //code
        for (pc, c) in self.code.iter().enumerate() {
            let mut line: u32 = '-' as u32;
            if !self.line_info.is_empty() {
                line = self.line_info[pc];
            }
            writeln!(f, "\t{}\t[{}]\t{:#010x}", pc + 1, line, c)?;
        }
        Ok(())
    }

    fn write_detail(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //detail
        writeln!(f, "constants ({}):", self.constants.len())?;
        for (i, k) in self.constants.iter().enumerate() {
            write!(f, "\t{}\t{}", i + 1, k)?;
        }
        writeln!(f, "locals ({}):", self.loc_vars.len())?;
        for (i, k) in self.loc_vars.iter().enumerate() {
            writeln!(f, "\t{}\t{}\t{}\t{}", i, k.var_name, k.start_pc, k.end_pc)?;
        }
        writeln!(f, "upvalues ({}):", self.upvalues.len())?;
        for (i, k) in self.upvalues.iter().enumerate() {
            writeln!(
                f,
                "\t{}\t{}\t{}\t{}",
                i, self.upvalue_names[i], k.instack, k.idx
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Nil => writeln!(f, "nil"),
            Constant::Boolean(b) => writeln!(f, "{}", b),
            Constant::Integer(i) => writeln!(f, "{}", i),
            Constant::Number(n) => writeln!(f, "{}", n),
            Constant::Str(s) => writeln!(f, "\"{}\"", s),
        }
    }
}

impl fmt::Display for Prototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_header(f)?;
        self.write_code(f)?;
        self.write_detail(f)?;
        for p in self.protos.iter() {
            p.fmt(f)?;
        }
        Ok(())
    }
}

pub fn undump(data: Vec<u8>) -> Prototype {
    let mut reader = Reader::new(data);
    reader.check_header();
    reader.read_byte();
    reader.read_proto("")
}
