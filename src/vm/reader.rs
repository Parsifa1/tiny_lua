use crate::vm::chunk::*;
pub struct Reader {
    data: Vec<u8>,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Reader {
        Reader { data }
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[0];
        self.data = self.data[1..].to_vec();
        byte
    }

    fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_byte());
        }
        result
    }

    fn read_u32(&mut self) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (self.read_byte() as u32) << (i * 8);
        }
        result
    }

    fn read_u64(&mut self) -> u64 {
        let mut result = 0;
        for i in 0..8 {
            result |= (self.read_byte() as u64) << ((i) * 8);
        }
        result
    }

    fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    fn read_lua_number(&mut self) -> f64 {
        let x = self.read_u64();
        f64::from_bits(x)
    }

    fn read_string(&mut self) -> String {
        let mut size = self.read_byte() as usize;
        if size == 0 {
            return "".to_string();
        }
        if size == 0xFF {
            size = self.read_u64() as usize;
        }
        let mut result = Vec::new();
        for _ in 0..size - 1 {
            result.push(self.read_byte());
        }
        String::from_utf8(result).unwrap()
    }

    fn read_code(&mut self) -> Vec<u32> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_u32());
        }
        result
    }

    fn read_constant(&mut self) -> Constant {
        let tag = self.read_byte();
        match tag {
            0x00 => Constant::Nil,
            0x01 => Constant::Boolean(self.read_byte() != 0),
            0x03 => Constant::Number(self.read_lua_number()),
            0x13 => Constant::Integer(self.read_lua_integer()),
            0x04 => Constant::Str(self.read_string()),
            0x14 => Constant::Str(self.read_string()),
            _ => panic!("Invalid constant tag!"),
        }
    }

    fn read_constants(&mut self) -> Vec<Constant> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_constant());
        }
        result
    }

    fn read_upvalues(&mut self) -> Vec<Upvalue> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(Upvalue {
                instack: self.read_byte(),
                idx: self.read_byte(),
            });
        }
        result
    }

    fn read_line_info(&mut self) -> Vec<u32> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_u32());
        }
        result
    }

    fn read_loc_vars(&mut self) -> Vec<LocVar> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(LocVar {
                var_name: self.read_string(),
                start_pc: self.read_u32(),
                end_pc: self.read_u32(),
            });
        }
        result
    }

    fn read_upvalue_names(&mut self) -> Vec<String> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_string());
        }
        result
    }

    pub fn check_header(&mut self) -> Header {
        Header {
            signature: self.read_bytes(4).try_into().expect("Invalid signature"),
            version: self.read_byte(),
            format: self.read_byte(),
            luac_data: self.read_bytes(6).try_into().expect("Invalid luac data"),
            cint_size: self.read_byte(),
            sizet_size: self.read_byte(),
            instruction_size: self.read_byte(),
            lua_integer_size: self.read_byte(),
            lua_number_size: self.read_byte(),
            luac_int: self.read_lua_integer(),
            luac_num: self.read_lua_number(),
        }
    }

    pub fn read_proto(&mut self, parent_source: &str) -> Prototype {
        let mut source = self.read_string();
        if source.is_empty() {
            source = parent_source.to_string();
        }
        Prototype {
            source: source.clone(),
            line_defined: self.read_u32(),
            last_line_defined: self.read_u32(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_code(),
            constants: self.read_constants(),
            upvalues: self.read_upvalues(),
            protos: self.read_protos(&source),
            line_info: self.read_line_info(),
            loc_vars: self.read_loc_vars(),
            upvalue_names: self.read_upvalue_names(),
        }
    }

    fn read_protos(&mut self, parent_source: &str) -> Vec<Prototype> {
        let size = self.read_u32() as usize;
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_proto(parent_source));
        }
        result
    }
}
