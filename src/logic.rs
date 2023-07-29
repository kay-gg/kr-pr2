use unic_ucd_hangul::{compose_syllable, decompose_syllable};

pub fn hangul() -> String {
    let mut args: Vec<String> = std::env::args().collect();
    //removes exec path
    args.remove(0);
    return args[0].clone();
}

// decomposes korean string into hangul components
pub fn decomp(hangul: String) -> Vec<Vec<char>> {
    let hanc_vec: Vec<char> = hangul.chars().collect();
    let mut decomp_vec: Vec<Vec<char>> = Vec::new();

    for han_char in hanc_vec {
        //taken from unic hangul docs
        let mut decomposed: Vec<char> = vec![];
        {
            let mut collect_decomposed = |chr: char| {
                decomposed.push(chr);
            };
            decompose_syllable(han_char, &mut collect_decomposed);
        }
        decomp_vec.push(decomposed.clone());
    }
    return decomp_vec;
}

//
pub fn pronounce(decomp: Vec<Vec<char>>) -> String {
    let mut new_decomp = decomp.clone();

    for window in decomp.windows(2) {
        // would be cool to move into a windows.filter closure
        // tests for if first hangul block has 3

        if window[0].len() == 3 {
            let trailing = *window[0].get(2).unwrap();
            let leading = *window[1].get(0).unwrap();

            let final_han = rules(trailing, leading);

            for i in 0..(new_decomp.len() - 1) {
                if new_decomp[i] == *window[0] {
                    new_decomp[i][2] = final_han.0;
                    new_decomp[i + 1][0] = final_han.1;
                }
            }
            new_decomp = filter_x(&mut new_decomp);
        }
    }

    return compose(new_decomp);
}

// changes hangul in original string to new hangul depending on these rules
fn rules(trailing: char, leading: char) -> (char, char) {
    // trailing consonant of the first window
    // leading consonant of the second window
	// 강호  ㅎ leading, ㅇ trailing
    let mut new_chars: (char, char) = (trailing, leading);
    //giyeok ᆨ ᆩ ᆿ ᆰ
    //'ᆨ'|'ᆩ'|'ᆿ'|'ᆰ'

    // x means to remove that char
    match leading {
        //example 입학 -> 이팍
        'ᄒ' => match trailing {
            'ᆨ' => {
                new_chars.0 = 'x';
                new_chars.1 = 'ᄏ';
            }
            'ᆸ' => {
                new_chars.0 = 'x';
                new_chars.1 = 'ᄑ';
            }
            'ᆮ' | 'ᆽ' | 'ᆺ' | 'ᆻ' => {
                new_chars.0 = 'x';
                new_chars.1 = 'ᄐ';
            }
            _ => {
                new_chars.0 = trailing;
                new_chars.1 = leading;
            }
        },
        'ᄂ' => match trailing {
            'ᆨ' | 'ᆩ' | 'ᆿ' => {
                new_chars.0 = 'ᆼ';
                new_chars.1 = 'ᄂ';
            },
            'ᆮ' | 'ᆽ' | 'ᆺ' | 'ᆻ' => {
                new_chars.0 = 'ᆫ';
                new_chars.1 = 'ᄂ';
            }
            'ᆸ' | 'ᇁ' | 'ᆲ' | 'ᆵ' => {
                new_chars.0 = 'ᆷ';
                new_chars.1 = 'ᄂ';
            },
            _ => {
                new_chars.0 = trailing;
                new_chars.1 = leading;
            } 
        },
        'ᄆ' => match trailing {
            'ᆨ' | 'ᆩ' | 'ᆿ' => {
                new_chars.0 = 'ᆼ';
                new_chars.1 = 'ᄆ';
            }
            'ᆮ' | 'ᆽ' | 'ᆺ' | 'ᆻ' => {
                new_chars.0 = 'ᆫ';
                new_chars.1 = 'ᄆ';
            }
            'ᆸ' | 'ᇁ' | 'ᆲ' | 'ᆵ' => {
                new_chars.0 = 'ᆷ';
                new_chars.1 = 'ᄆ';
            }


            _ => {
                new_chars.0 = trailing;
                new_chars.1 = leading;
            } 
        }
        'ᄅ' => match trailing {
            'ᆷ' => {
                new_chars.0 = 'ᆷ';
                new_chars.1 = 'ᄂ';
            }
            'ᆼ' => {
                new_chars.0 = 'ᆼ';
                new_chars.1 = 'ᄂ';
            }
            'ᆨ' => {
                new_chars.0 = 'ᆼ';
                new_chars.1 = 'ᄂ';
            }
            'ᆸ' => {
                new_chars.0 = 'ᆷ';
                new_chars.1 = 'ᄂ';
            }
            'ᆫ' => {
                new_chars.0 = 'ᆯ';
                new_chars.1 = 'ᄅ';
            }
            _ => {
                new_chars.0 = trailing;
                new_chars.1 = leading;
            } 
        }
        'ᄀ' => match trailing {
            _ => {
                new_chars.0 = trailing;
                new_chars.1 = leading;
            } 
        }

        // no rules to apply. just return the same jamos
        _ => {
            new_chars.0 = trailing;
            new_chars.1 = leading;
        }
    }

    return new_chars;
}

// filters out blank chars. 학호 -> 하코
fn filter_x(decomposed: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..decomposed.len() {
        for j in 0..decomposed[i].len() {
            if decomposed[i][j] == 'x' {
                decomposed[i].remove(j);
            }
        }
    }

    return decomposed.to_vec();
}

// composes the decomposed, and correctly pronounced hanguls into full hangul blocks
fn compose(decomposed: Vec<Vec<char>>) -> String {
    let mut final_string = String::new();

    for v in decomposed {
        let mut composed = compose_syllable(v[0], v[1]).unwrap();
        if v.len() == 3 {
            composed = compose_syllable(composed, v[2]).unwrap();
        }
        final_string.push(composed);
    }

    return final_string;
}