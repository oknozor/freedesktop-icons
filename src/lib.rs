pub mod error;
pub mod lookup;
pub mod theme;

use std::path::PathBuf;
pub use error::Error;

const HICOLOR: &str = "hicolor";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", dirs::data_dir());
    }
}
