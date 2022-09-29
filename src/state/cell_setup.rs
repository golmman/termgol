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

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CellSetup {
    Acorn,
    Blank,
    RPentonimo,
    Termgol,
}

impl From<CellSetup> for &str {
    fn from(cell_setup: CellSetup) -> Self {
        match cell_setup {
            CellSetup::Acorn => ACORN,
            CellSetup::Blank => BLANK,
            CellSetup::RPentonimo => R_PENTONIMO,
            CellSetup::Termgol => TERMGOL,
        }
    }
}
