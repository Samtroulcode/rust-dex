#[derive(Debug)]
pub struct Pokemon {
    pub nom: String,
    pub p_type: String,
}

impl Pokemon {
    pub fn new(nom: &str, p_type: &str) -> Self {
        Self {
            nom: nom.to_string(),
            p_type: p_type.to_string(),
        }
    }
}
