const INPUT_VALUES: &'static str = include_str!("../day03");

fn char_to_value(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        u32::from(ch) - 96
    } else {
        u32::from(ch) - 38
    }
}

fn main() {

    let mut result : u32 = 0;
    for line in INPUT_VALUES.lines() {
        let splitted = line.split_at(line.len() / 2);
        // println!("Splited into:\n {} \t {}", splitted.0, splitted.1);
        for char in splitted.0.chars() {
            if splitted.1.contains(char) {
                let value = char_to_value(char);
                result += value;
                break;
            }
        }
    }
    println!("Sum is {}",result);
    // part 2
    let mut lines = INPUT_VALUES.lines();
    let mut result_part_2 : u32 = 0;
    while let Some(base_line) = lines.next() {
        let lines_to_check = (lines.next().unwrap(), lines.next().unwrap());
        let id = base_line.chars().into_iter().find(|&ch| lines_to_check.0.contains(ch) && lines_to_check.1.contains(ch));
        if let Some(badge) = id {
            result_part_2 += char_to_value(badge);
            // println!("Badge is {} with value of {}", badge, char_to_value(badge));
        }
    }
    println!("Sum for badges: {}",result_part_2);

}
