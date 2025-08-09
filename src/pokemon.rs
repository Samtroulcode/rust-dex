#[derive(Debug)]
pub struct Pokemon {
    pub p_id: usize,
    pub nom: String,
    pub p_type: String,
}

impl Pokemon {
    pub fn new(nom: &str, p_type: &str, id: usize) -> Self {
        Self {
            p_id: id,
            nom: nom.to_string(),
            p_type: p_type.to_string(),
        }
    }
}
