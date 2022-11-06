pub struct Section {
        pub name: std::string::String,
        pub section_type: SectionType,
        pub vma: u64,
        pub size: u64,
        pub bytes: Vec<u8>,
}
    
pub enum SectionType {
    SecTypeNone = 0,
    SecTypeCode = 1,
    SecTypeData = 2
}

impl SectionType {
    pub fn contains(section: Section, address: u64) -> bool {
        return (address >= section.vma) && (address-section.vma < section.size);
    }
}