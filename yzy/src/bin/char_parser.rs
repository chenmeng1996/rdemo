fn main() {
    // println!("char_parser start");
    parse("abc d陈\r\n\x00");
}

fn parse(s: &str) {
    let cr = b'\x0D';
    let lf = b'\x0A';
    let vchar_min = b'\x21';
    let vchar_max = b'\x7E';
    let wsp = [b'\x20', b'\x09'];
    let special_unvisible = b'\x00';

    let mut visible_char: Vec<u8> = Vec::new();
    visible_char.push(cr);
    visible_char.push(lf);
    let mut i = vchar_min;
    while i <= vchar_max {
        visible_char.push(i);
        i += 1;
    }
    for i in wsp.iter() {
        visible_char.push(*i);
    }
    // println!("visible_char_set: {:#?}\n", visible_char);
    let mut s_len = s.len();
    let mut visible_count = 0;
    let mut unvisible_count = 0;
    let mut special_unvisible_count = 0;
    for item in s.as_bytes() {
        if visible_char.contains(item) {
            visible_count += 1;
        } else {
            unvisible_count += 1;
            if *item == special_unvisible {
                special_unvisible_count += 1;
            }
        }
    }
    let mut visible_char_rate = 0.0;
    let mut special_unvisible_char_rate = 0.0;
    visible_char_rate = visible_count as f64 / s_len as f64;
    special_unvisible_char_rate = special_unvisible_count as f64 / unvisible_count as f64 ;
    println!("total: {}\nvisible: {}\nunvisible: {}\nspecial_unvisible: {}\n", s_len, visible_count, unvisible_count, special_unvisible_count);
    println!("visible_char_rate: {}\nspecial_unvisible_char_rate: {}", visible_char_rate, special_unvisible_char_rate);
}