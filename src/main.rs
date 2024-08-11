mod vm;

use crate::vm::chunk::undump;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_name = &args[1];
        let data = std::fs::read(file_name).unwrap();
        let proto = undump(data);
        println!("{}", proto);
    }
}

#[test]
fn check() {
    use crate::vm::chunk::*;
    use crate::vm::reader::Reader;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_name = &args[1];
        let data = std::fs::read(file_name).unwrap();
        let mut reader = Reader::new(data);
        let header = reader.check_header();
        assert_eq!(header.signature, *LUA_SIGNATURE);
        assert_eq!(header.version, LUAC_VERSION);
        assert_eq!(header.format, LUAC_FORMAT);
        assert_eq!(header.luac_data, *LUAC_DATA);
        assert_eq!(header.cint_size, CINT_SIZE);
        assert_eq!(header.sizet_size, CSIZET_SIZE);
        assert_eq!(header.instruction_size, INSTRUCTION_SIZE);
        assert_eq!(header.lua_integer_size, LUA_INTEGER_SIZE);
        assert_eq!(header.lua_number_size, LUA_NUMBER_SIZE);
        assert_eq!(header.luac_int, LUAC_INT);
        assert_eq!(header.luac_num, LUAC_NUM);
    }
}
