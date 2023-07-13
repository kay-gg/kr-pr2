extern crate unic_ucd_hangul;
mod logic;
mod output;
pub struct HanInfo {
    pub hangul:         String,
    pub decomp:         Vec<Vec<char>>,
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
fn create_info() -> HanInfo {
    let hangul = logic::hangul();
    let decomp = logic::decomp(hangul.clone());
    let pronounced_as = logic::pronounce(decomp.clone());
    
    let info = HanInfo {
        hangul,
        decomp,
        pronounced_as,
    };

    return info;
}

fn arg_tests(args: &mut Vec<String>) {
    assert!(!args.is_empty(), "You didn't enter a word.");
    // args start with path to executable, so len is 2
    assert_eq!(args.len(), 1, "Enter 1 Korean word.")
}