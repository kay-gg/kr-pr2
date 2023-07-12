use unic_ucd_hangul::{decompose_syllable, compose_syllable};

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
		//taken from unic hangul docs
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

pub fn pronounce(decomp: Vec<Vec<char>>) -> String {
	let mut new_decomp = decomp.clone();

	for window in decomp.windows(2) {
		// would be cool to move into a windows.filter closure
		// tests for if first hangul block has 3 

		if window[0].len() == 3  {
			let trailing = *window[0].get(2).unwrap();
			let leading = *window[1].get(0).unwrap();
			
			let final_han = rules(trailing, leading);

			for i in 0..(new_decomp.len() - 1) {
				if new_decomp[i] == *window[0] {
					new_decomp[i][2] = final_han.0;
					new_decomp[i+1][0] = final_han.1;
				}
			}
			// filters out blank chars. 학호 -> 하코
			new_decomp = filter_x(&mut new_decomp);
		}
	}
	
	// testing 
	// let mut zz = 0;
	// for ss in new_decomp {
	// 	println!("\n{}", zz);
	//     for sssss in ss {
	// 	  println!("{}", sssss);
	//     }
	// 	zz = zz + 1;
	// }
	return compose(new_decomp);
}
fn rules(trailing: char, leading: char) -> (char, char) {
	// trailing consonant of the first window
	// leading consonant of the second window
	let mut new_chars: (char, char) = (trailing, leading);
	//giyeok ᆨ ᆩ ᆿ ᆰ
	//'ᆨ'|'ᆩ'|'ᆿ'|'ᆰ'

	// x means to remove that char
	match leading {
		'ᄒ' => {
			match trailing {
				'ᆨ' => {new_chars.0 = 'x'; new_chars.1 ='ᄏ'},
				'ᆸ' => new_chars.0 = 'ᇁ',
				'ᆮ'|'ᆽ' => new_chars.0 = 'ᆾ',
				_ => todo!(),
			}
	    	},
	    'ᆯ' => todo!(),
	    'ᆫ'|'ᆷ' => todo!(),

	    _ => todo!(),
	}

	return new_chars;
}

fn filter_x(decomposed: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
	for i in 0..decomposed.len() {
		for j in 0..decomposed[i].len() {
			if decomposed[i][j] == 'x' {
				decomposed[i].remove(j);
			}
		}
	}
	//let immut: Vec<Vec<char>> = decomposed.into_iter().map(|x| *x).collect();

	return decomposed.to_vec();
}

fn compose(decomposed: Vec<Vec<char>>) -> String {
	let mut final_string = String::new();

	for v in decomposed {
		let mut composed: char = '\0';

		for ch in 0..v.len()-1 {
			composed = compose_syllable(v[ch], v[ch+1]).unwrap();
		}
		final_string.push(composed);
	}
	return final_string;
}