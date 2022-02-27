use alloc::borrow::Cow;

use self::custom_section::CustomSection;

pub mod code_section;
pub mod custom_section;
pub mod data_section;
pub mod errors;
pub mod export_section;
pub mod function_section;
pub mod global_section;
pub mod import_section;
pub mod magic_section;
pub mod memory_section;
pub mod start_section;
pub mod table_section;
pub mod type_section;
pub mod types;

pub use self::{
    code_section::*, custom_section::*, data_section::*, export_section::*, function_section::*,
    global_section::*, import_section::*, magic_section::*, memory_section::*, start_section::*,
    table_section::*, type_section::*,
};

#[derive(Clone, Debug)]
pub struct WasmModule<'a> {
    pub magic_section: MagicSection<'a>,
    pub custom_section: Option<CustomSection<'a>>,
    pub type_section: Option<TypeSection<'a>>,
}

/*
Each section consists of:
1. a one-byte section id,
2. the size of the contents, in bytes,
3. the actual contents, whose structure is depended on the section id.
*/

impl<'a> WasmModule<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        // handle magic section first
        let magic_sec = Self::magic_section(&bytes[..8]);
        let bytes = &bytes[8..];
        loop {
            // the start of every section is id.
            let section_id = bytes[0];
            match section_id {
                // Custom section starts with 0
                0 => (),
                // Type section starts with 1
                1 => (),
                // Import section starts with 2
                2 => (),
                // Function section starts with 3
                3 => (),
                // Table section starts with 4
                4 => (),
                // Memory section starts with 5
                5 => (),
                // Global section starts with 6
                6 => (),
                // Export section starts with 7
                7 => (),
                // Start section starts with 8
                8 => (),
                // Element section starts with 9
                9 => (),
                // Code section starts with 10
                10 => (),
                // Data section starts with 11
                11 => (),
                // Data count section starts with 12
                12 => (),
                _ => {
                    println!("Unsupported section");
                }
            }

            if section_id >= 11 {
                break;
            }
        }
        todo!();
    }

    fn magic_section(bytes: &'a [u8]) -> MagicSection<'a> {
        MagicSection::from_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_le() {
        let a = 892u32;
        dbg!(a.to_le_bytes());
        dbg!(a.to_be_bytes());
    }
}
