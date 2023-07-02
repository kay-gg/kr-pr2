pub fn hangul() -> String {
	let mut args: Vec<String> = std::env::args().collect();
	//removes exec path
	args.remove(0);
	return args[0].clone();	
}

pub fn decomp() -> Vec<char> {

todo!()
}

pub fn pronounce() -> String {

todo!()
}