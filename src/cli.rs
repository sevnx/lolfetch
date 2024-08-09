use lolfetch_ascii::AsciiColoredChar;

pub mod image;

pub struct Layout {
    logo: Vec<Vec<AsciiColoredChar>>,
    info: Vec<String>,
}

impl Layout {
    pub fn new(logo: Vec<Vec<AsciiColoredChar>>, info: Vec<String>) -> Self {
        Self { logo, info }
    }

    pub fn display(&self) {
        let mut logo_lines = self.logo.iter();
        let mut info_lines = self.info.iter();
        let center_pad = 10;

        loop {
            match (logo_lines.next(), info_lines.next()) {
                (Some(logo_line), Some(info_line)) => {
                    logo_line.iter().for_each(|i| print!("{i}"));
                    println!("{center_pad}{info_line:^}")
                }
                (None, Some(info_line)) => {
                    println!("{info_line}")
                }
                (Some(logo_line), None) => {
                    logo_line.iter().for_each(|i| print!("{i}"));
                    println!("");
                }
                (None, None) => {
                    break;
                }
            }
        }
    }
}
