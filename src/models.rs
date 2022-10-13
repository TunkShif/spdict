#[derive(Debug)]
pub struct Word {
    pub name: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug)]
pub struct Definition {
    pub pos: String,
    pub sense: String,
    pub examples: Vec<Example>,
}

#[derive(Debug)]
pub struct Example {
    pub es_text: String,
    pub en_text: String,
}

#[derive(Debug)]
pub struct Conjugation {}
