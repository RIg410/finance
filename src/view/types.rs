use std::fmt::Display;

pub struct TypeView {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl Display for TypeView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{}'; {}", self.id, self.name, self.description)
    }
}
