#[derive(Debug)]
pub struct SymbolTable {
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            entries: Vec::<(String, f64)>::new(),
        }
    }
    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        if self
            .entries
            .iter()
            .find(|item| item.0 == identifier)
            .is_some()
        {
            Err(format!(
                "Error: Identifier '{}' declared several times.",
                identifier
            ))
        } else {
            self.entries.push((identifier.to_string(), 0.));
            Ok(self.entries.len() - 1)
        }
    }
    pub fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        if let Some(pos) = self.entries.iter().position(|item| item.0 == identifier) {
            Ok(pos)
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
}
