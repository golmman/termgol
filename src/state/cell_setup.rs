use std::{fs::File, io::Read};

use clap::Error;

const ACORN: &str = r"
 #     
   #   
##  ###
";

const BLANK: &str = "";

const R_PENTONIMO: &str = r"
## 
 ##
 # 
";

const TERMGOL: &str = r"
##### ##### ####  #   #  ####  ###  #    
  #   #     #   # ## ## #     #   # #    
  #   ####  ####  # # # #  ## #   # #    
  #   #     #  #  #   # #   # #   # #    
  #   ##### #   # #   #  ###   ###  #####
";

#[derive(Clone, Debug)]
pub enum CellSetup {
    Acorn,
    Blank,
    RPentonimo,
    Termgol,
    FileContent(String),
}

impl From<CellSetup> for String {
    fn from(cell_setup: CellSetup) -> Self {
        match cell_setup {
            CellSetup::Acorn => ACORN.to_string(),
            CellSetup::Blank => BLANK.to_string(),
            CellSetup::RPentonimo => R_PENTONIMO.to_string(),
            CellSetup::Termgol => TERMGOL.to_string(),
            CellSetup::FileContent(s) => s,
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
            _ => {
                let mut file = File::open(s)?;
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;

                CellSetup::FileContent(file_content)
            }
        };

        Ok(cell_setup)
    }
}
