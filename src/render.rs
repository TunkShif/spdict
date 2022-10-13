use crate::models::*;
use colored::*;

pub trait Render {
    fn render(&self) -> String;
}

impl Render for Word {
    fn render(&self) -> String {
        let mut str = String::with_capacity(256);
        str.push_str(
            &format!(" {} ", &self.name)
                .bold()
                .truecolor(255, 255, 255)
                .on_truecolor(55, 160, 234)
                .to_string(),
        );
        str.push_str("\n\n");
        self.definitions
            .iter()
            .for_each(|def| str.push_str(&def.render()));
        str
    }
}

impl Render for Definition {
    fn render(&self) -> String {
        let mut str = String::with_capacity(256);
        str.push(' ');
        str.push_str(
            &format!(" {} ", &self.pos)
                .bold()
                .truecolor(255, 255, 255)
                .on_truecolor(55, 160, 234)
                .to_string(),
        );
        str.push(' ');
        str.push_str(&self.sense);
        str.push('\n');
        self.examples
            .iter()
            .for_each(|ex| str.push_str(&ex.render()));
        str.push('\n');
        str
    }
}

impl Render for Example {
    fn render(&self) -> String {
        let mut str = String::with_capacity(128);
        str.push_str("  • ");
        str.push_str(&self.es_text);
        str.push_str("\n    ");
        str.push_str(&self.en_text.bright_black().to_string());
        str.push('\n');
        str
    }
}
