pub struct Section {
        pub name: std::string::String,
        pub section_type: SectionType,
        pub vm_address: u64,
        pub size: u64,
        pub bytes: Vec<u8>,
    }
    
    pub enum SectionType {
        NONE,
        CODE,
        DATA,
    }
    
    impl SectionType {
        pub fn contains(_address: u64) -> bool {
            true
        }
    }