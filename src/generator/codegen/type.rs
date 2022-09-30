use super::*;

pub fn gen_rs_for_type(_: &mut Generator, typ: Type) -> String {
    match typ {
        Type::FeStr => return String::from("FeStr"),
        _ => todo!(),
    }
}

