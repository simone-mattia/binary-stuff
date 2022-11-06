pub enum Arch {
    ArchNone = 0,
    ArchX86 = 1
}

pub enum BinaryType {
    BinTypeAuto = 0,
    BinTypeElf = 1,
    BinTypePe = 2
}

pub struct Binary {
    pub filename: std::string::String,
    pub binary_type: BinaryType,
    pub type_str: std::string::String,
    pub arch: Arch,
    pub arch_str: std::string::String,
    pub bits: u8,
    pub entry_point: u64,
    pub sections: Vec<super::section::Section>,
    pub symbols: Vec<super::symbol::Symbol>,
}

pub fn get_text_section(bin: Binary) -> super::section::Section{
    for section in bin.sections{
        if section.name == ".text" {
            return section;
        }
    }
    return super::section::Section{
        name: "".to_string(),
        section_type: super::section::SectionType::SecTypeNone,
        vma: 0,
        size: 0,
        bytes:vec![0]
    }
}