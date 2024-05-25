// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use std::{fs::File, io::{Error, Write}};

pub mod model;

pub fn create_file<T>(path: &str, data: &[T]) -> Result<(),Error> {
    let mut f = File::create(path)?;
    f.write_all(unsafe {slicebytes::cast_bytes(data)})?;
    Ok(())
}