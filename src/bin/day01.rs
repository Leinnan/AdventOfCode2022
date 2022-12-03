const INPUT_VALUES: &'static str = include_str!("../day01");

fn main() {
    let mut values = Vec::new();
    let mut elf_value = 0;
    for line in INPUT_VALUES.lines() {
        if line.is_empty() {
            values.push(elf_value);
            elf_value = 0;
            continue;
        }
        elf_value = elf_value + line.parse::<i32>().unwrap();
    }

    let index: Option<usize> = values
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index);
    println!("Elf nr {} has the highest calories index. How many total Calories is that Elf carrying? {}", index.unwrap(), values[index.unwrap()]);

    values.sort();
    let top_x_elfs: i32 = values.iter().rev().take(3).sum();
    println!("Top three elfs are carrying: {}", top_x_elfs);
}
