use super::*;

pub fn gen_rs_for_type(generator: &mut Generator, typ: Type) -> String {
    match typ {
        Type::FeStr => return String::from("FeStr"),
        Type::SharedRef(inner) => return format!("&{}", gen_rs_for_type(generator, *inner)),
        Type::MutRef(inner) => return format!("&mut {}", gen_rs_for_type(generator, *inner)),
        _ => todo!(),
    }
}

