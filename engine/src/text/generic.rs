use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use num_traits::Float;
use crate::fileio::image::raw::RawImage;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::text::TextConfig;

static ALPHABET : LazyLock<Mutex<HashMap<char, Vec<u16>>>> = LazyLock::new(|| Mutex::new(define_alphabet()));
static HEIGHT: usize = 15;
static WIDTH: usize = 13;

pub fn create_generic<F: Float>(config: &TextConfig<F>, message: String) -> RawImage {
    /* make output for the appended letters */
    let mut rows: Vec<Vec<u16>> = Vec::with_capacity(HEIGHT);
    for _ in 0..HEIGHT {
        rows.push(vec!());
    }

    /* get all the appended letters, as u16 */
    for letter in message.chars() {
        let letter_data = find(letter);
        for (idx, word) in letter_data.iter().enumerate() {
            let row: &mut Vec<u16> = &mut rows[idx];
            row.push(word.reverse_bits());
        }
    }

    //log(LogLevel::Error, &|| String::from(format!("[{}] rows; should be equal to [{}]", rows.len(), HEIGHT)));
    //for i in 0..HEIGHT {
    //    log(LogLevel::Error, &|| String::from(format!("row[{}] has [{}] elements", i, rows[i].len())));
    //}

    /* create storage for u8 */
    let mut output = vec!();

    /* iterate over u16 and convert to u8 */
    for row in rows {
        for word in row {
            for i in 0..WIDTH {
                let bit = ((word >> i) & 1) != 0;

                if i == 0 && !bit {
                    log(LogLevel::Error, &|| String::from(format!("WHY IS THE BIT [{}]", bit as u32)));
                }

                let expanded = if bit {
                    config.foreground.to_u8()
                } else {
                    config.background.to_u8()
                };

                output.push(expanded.0);
                output.push(expanded.1);
                output.push(expanded.2);
                output.push(expanded.3);
            }
        }
    }

    /* done */
    RawImage::new((message.len() * WIDTH) as u32, HEIGHT as u32, output)
}

fn find(letter: char) -> Vec<u16> {
    let map = ALPHABET.lock().unwrap();
    if map.contains_key(&letter) {
        map.get(&letter).unwrap().to_vec()
    } else {
        map.get(&' ').unwrap().to_vec()
    }
}

fn define_alphabet() -> HashMap<char, Vec<u16>> {
    let mut data = HashMap::new();

    data.insert(' ', vec![
        0b1111111111111111,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1000000000000001,
        0b1111111111111111,
    ]);
    //data.insert('!',  vec!());
    //data.insert('"',  vec!());
    //data.insert('#',  vec!());
    //data.insert('$',  vec!());
    //data.insert('%',  vec!());
    //data.insert('&',  vec!());
    //data.insert('\'', vec!());
    //data.insert('(',  vec!());
    //data.insert(')',  vec!());
    //data.insert('*',  vec!());
    //data.insert('+',  vec!());
    //data.insert(',',  vec!());
    //data.insert('-',  vec!());
    //data.insert('.',  vec!());
    //data.insert('/',  vec!());
    //data.insert('0',  vec!());
    //data.insert('1',  vec!());
    //data.insert('2',  vec!());
    //data.insert('3',  vec!());
    //data.insert('4',  vec!());
    //data.insert('5',  vec!());
    //data.insert('6',  vec!());
    //data.insert('7',  vec!());
    //data.insert('8',  vec!());
    //data.insert('9',  vec!());
    //data.insert(':',  vec!());
    //data.insert(';',  vec!());
    //data.insert('<',  vec!());
    //data.insert('=',  vec!());
    //data.insert('>',  vec!());
    //data.insert('?',  vec!());
    //data.insert('@',  vec!());
    data.insert('A',  vec![
        0b1111111111111000,
        0b1000111000001000,
        0b1000001000001000,
        0b1000010100001000,
        0b1000010100001000,
        0b1000100010001000,
        0b1000100010001000,
        0b1000111110001000,
        0b1001000001001000,
        0b1001000001001000,
        0b1011100011101000,
        0b1000000000001000,
        0b1000000000001000,
        0b1000000000001000,
        0b1111111111111000,
    ]);
    //data.insert('B',  vec!());
    //data.insert('C',  vec!());
    //data.insert('D',  vec!());
    //data.insert('E',  vec!());
    //data.insert('F',  vec!());
    //data.insert('G',  vec!());
    //data.insert('H',  vec!());
    //data.insert('I',  vec!());
    //data.insert('J',  vec!());
    //data.insert('K',  vec!());
    //data.insert('L',  vec!());
    //data.insert('M',  vec!());
    //data.insert('N',  vec!());
    //data.insert('O',  vec!());
    //data.insert('P',  vec!());
    //data.insert('Q',  vec!());
    //data.insert('R',  vec!());
    //data.insert('S',  vec!());
    //data.insert('T',  vec!());
    //data.insert('U',  vec!());
    //data.insert('V',  vec!());
    //data.insert('W',  vec!());
    //data.insert('X',  vec!());
    //data.insert('Y',  vec!());
    //data.insert('Z',  vec!());
    //data.insert('[',  vec!());
    //data.insert('\\', vec!());
    //data.insert(']',  vec!());
    //data.insert('^',  vec!());
    //data.insert('_',  vec!());
    //data.insert('`',  vec!());
    //data.insert('a',  vec!());
    //data.insert('b',  vec!());
    //data.insert('c',  vec!());
    //data.insert('d',  vec!());
    //data.insert('e',  vec!());
    //data.insert('f',  vec!());
    //data.insert('g',  vec!());
    //data.insert('h',  vec!());
    //data.insert('i',  vec!());
    //data.insert('j',  vec!());
    //data.insert('k',  vec!());
    //data.insert('l',  vec!());
    //data.insert('m',  vec!());
    //data.insert('n',  vec!());
    //data.insert('o',  vec!());
    //data.insert('p',  vec!());
    //data.insert('q',  vec!());
    //data.insert('r',  vec!());
    //data.insert('s',  vec!());
    //data.insert('t',  vec!());
    //data.insert('u',  vec!());
    //data.insert('v',  vec!());
    //data.insert('w',  vec!());
    //data.insert('x',  vec!());
    //data.insert('y',  vec!());
    //data.insert('z',  vec!());
    //data.insert('{',  vec!());
    //data.insert('|',  vec!());
    //data.insert('}',  vec!());
    //data.insert('~',  vec!());

    data
}
