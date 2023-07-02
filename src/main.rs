extern crate unic_ucd_hangul;
// unic_ucd_hangul gives these letters 
// https://unicode-table.com/en/blocks/hangul-jamo/
// rather than the normal hangul
mod logic;
mod output;
pub struct HanInfo {
    pub hangul:         String,
    pub decomp:         Vec<char>,
    pub pronounced_as:  String,
}

fn main() {
    //code path
    //take input
        //parse args (no args for now)
        //input will be one korean word. no spaces.
    //make new string of how to pronounce it
        //bunch of shit
    //output
    let mut args: Vec<String> = std::env::args().collect();
    //removes exec path
    args.remove(0);
    arg_tests(&mut args);
    
    let info = create_info();
    output::default(info);
}

fn arg_tests(args: &mut Vec<String>) {
    assert!(!args.is_empty(), "You didn't enter a word.");
    // args start with path to executable, so len is 2
    assert_eq!(args.len(), 1, "Enter 1 Korean word.")
}

fn create_info() -> HanInfo {
    let info = HanInfo{
        hangul:         logic::hangul(),
        decomp:         logic::decomp(),
        pronounced_as:  logic::pronounce(),
    };

    return info;
}