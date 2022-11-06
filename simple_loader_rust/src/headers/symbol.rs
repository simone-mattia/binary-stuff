pub struct Symbol {
    pub symbol_type: SymbolType,
    pub name: std::string::String,
    pub address: u64,
}

pub enum SymbolType {
    SymTypeUnk = 0,
    SymTypeFunc = 1
}