use unic_ucd_hangul::decompose_syllable;

pub fn hangul() -> String {
	let mut args: Vec<String> = std::env::args().collect();
	//removes exec path
	args.remove(0);
	return args[0].clone();	
}

pub fn decomp(hangul: String) -> Vec<Vec<char>> {
	let hanc_vec: Vec<char> = hangul.chars().collect();
	let mut decomp_vec: Vec<Vec<char>> = Vec::new();

	for han_char in hanc_vec {
		let mut decomposed: Vec<char> = vec![]; {
			let mut collect_decomposed = |chr: char| {
			    decomposed.push(chr);
			};
			decompose_syllable(han_char, &mut collect_decomposed);
		}
		decomp_vec.push(decomposed.clone());
	}
	return decomp_vec;
}

pub fn pronounce() -> String {

todo!()
}