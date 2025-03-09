use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wrdlist")]
#[command(
    about = "Lightweight Wordlists Generator",
    override_usage = "wrdlist [OPTIONS] <PATTERN> [OPTIONAL]"
)]
#[command(
    help_template = "\
{about}

{usage}

Arguments:
  <PATTERN>  
      The pattern to generate the wordlist (must be enclosed in double quotes)

Options:
  -r, --random  
      Randomize the order of the wordlist  

  -i, --inverse  
      Reverse the order of the wordlist  

  -h, --help  
      Print help information

  -v, --version  
      Print the version

Pattern Syntax:
  - Use '.' with a character to keep it fixed in every word (`.a` means 'a' stays in place).
  - Use '[start-end]' for numeric ranges (`[1-3]` will generate 1, 2, 3).
  - Use '!' for lowercase letters (a-z).  
  - Use '@' for uppercase letters (A-Z).  
  - Use '#' for digits (0-9).  
  - Use '%' for symbols (!@#$%^&*?).  

Optional:
  -o, --output <OUTPUT>  
      Save the wordlist to a file 

Examples:
  wrdlist \".#[9-11].X.d\"  
  wrdlist -i \"[1-3].!\" -o output.txt  
  wrdlist -r \"[0-5].-[0-5]\" -o output.txt"
)]
pub struct Cli {
    /// The pattern to generate the wordlist (must be enclosed in double quotes)
    #[arg(required = true)]
    pub pattern: String,

    /// Output file to save the wordlist
    #[arg(short, long)]
    pub output: Option<String>,

    /// Randomize the order of the wordlist
    #[arg(short, long)]
    pub random: bool,

    /// Reverse the order of the wordlist
    #[arg(short, long)]
    pub inverse: bool,

    /// Print the version information
    #[arg(short, long)]
    pub version: bool,
}
