mod result;

pub use result::Result;

const RUNTIME_LIB_RS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/runtime/lib.rs"));

pub fn print_files() {
    println!("src/lib.rs:\n{RUNTIME_LIB_RS}\n");
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
