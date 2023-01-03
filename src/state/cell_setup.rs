use nanorand::Rng;
use nanorand::WyRand;
use std::fs::File;
use std::io::Read;

use clap::Error;
use regex::Regex;

const ACORN: &str = r"
.O.....
...O...
OO..OOO
";

const BLANK: &str = "";

const R_PENTONIMO: &str = r"
OO.
.OO
.O.
";

const TERMGOL: &str = r"
OOOOO.OOOOO.OOOO..O...O..OOOO..OOO..O....
..O...O.....O...O.OO.OO.O.....O...O.O....
..O...OOOO..OOOO..O.O.O.O..OO.O...O.O....
..O...O.....O..O..O...O.O...O.O...O.O....
..O...OOOOO.O...O.O...O..OOO...OOO. OOOOO
";

#[derive(Clone, Debug)]
pub enum CellSetup {
    Acorn,
    Blank,
    RPentonimo,
    Termgol,
    Special(String),
}

impl From<CellSetup> for String {
    fn from(cell_setup: CellSetup) -> Self {
        match cell_setup {
            CellSetup::Acorn => ACORN.to_string(),
            CellSetup::Blank => BLANK.to_string(),
            CellSetup::RPentonimo => R_PENTONIMO.to_string(),
            CellSetup::Termgol => TERMGOL.to_string(),
            CellSetup::Special(s) => s,
        }
    }
}

impl CellSetup {
    pub fn parse(s: &str) -> Result<CellSetup, Error> {
        let cell_setup = match s {
            "acorn" => CellSetup::Acorn,
            "blank" => CellSetup::Blank,
            "r-pentonimo" => CellSetup::RPentonimo,
            "termgol" => CellSetup::Termgol,
            _ => CellSetup::parse_special(s)?,
        };

        Ok(cell_setup)
    }

    fn parse_special(s: &str) -> std::io::Result<CellSetup> {
        let soup_regex = Regex::new(r"^soup(\d{1,3})$").unwrap();
        let mut captures_iter = soup_regex.captures_iter(s);
        if let Some(captures) = captures_iter.next() {
            let soup_size: i32 = captures[1].parse().unwrap();
            Ok(CellSetup::rect_soup(soup_size, soup_size))
        } else {
            let mut file = File::open(s)?;
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;
            Ok(CellSetup::Special(file_content))
        }
    }

    pub fn rect_soup(width: i32, height: i32) -> CellSetup {
        let mut soup = String::new();

        let mut rng = WyRand::new();
        for _y in 0..height {
            for _x in 0..width {
                if rng.generate_range(0_u8..=1) == 0 {
                    soup.push('.');
                } else {
                    soup.push('O');
                }
            }
            soup.push('\n');
        }

        CellSetup::Special(soup)
    }
}
