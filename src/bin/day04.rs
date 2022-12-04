fn convert_line_to_pair(line: &String) -> (Vec<u32>, Vec<u32>) {
    let pairs: Vec<&str> = line.split(',').collect();
    assert_eq!(pairs.len(), 2);
    let left_vector: Vec<u32> = pairs[0]
        .split('-')
        .map(String::from)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let right_vector: Vec<u32> = pairs[1]
        .split('-')
        .map(String::from)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    assert_eq!(left_vector.len(), right_vector.len());
    (left_vector, right_vector)
}

fn is_one_containing_another(one: &Vec<u32>, two: &Vec<u32>) -> bool {
    if one[0] < two[0] || one[1] > two[1] {
        two[0] >= one[0] && two[1] <= one[1]
    } else {
        one[0] >= two[0] && one[1] <= two[1]
    }
}

fn is_one_pair_containing_another(line: &String) -> bool {
    let (left_vector, right_vector) = convert_line_to_pair(line);
    let result = is_one_containing_another(&left_vector, &right_vector);

    // println!("{:?} vs {:?} = {result}", left_vector, right_vector);

    result
}
fn are_overlaping(line: &String) -> bool {
    let (left_vector, right_vector) = convert_line_to_pair(line);
    is_one_containing_another(&left_vector, &right_vector)
        || left_vector[0] >= right_vector[0] && left_vector[0] <= right_vector[1]
        || left_vector[1] >= right_vector[0] && left_vector[1] <= right_vector[1]
}

fn count_containing_pairs(input: &str) -> usize {
    input
        .lines()
        .map(String::from)
        .filter(is_one_pair_containing_another)
        .count()
}

fn count_overlaps_in_pairs(input: &str) -> usize {
    input
        .lines()
        .map(String::from)
        .filter(are_overlaping)
        .count()
}

#[test]
fn test_case() {
    const TEST_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(count_containing_pairs(TEST_INPUT), 2);
    assert_eq!(count_overlaps_in_pairs(TEST_INPUT), 4);
}
#[test]
fn main_case() {
    assert_eq!(count_containing_pairs(include_str!("../day04")), 448);
    assert_eq!(count_overlaps_in_pairs(include_str!("../day04")), 794);
}

fn main() {
    println!(
        "In how many assignment pairs does one range fully contain the other? {}",
        count_containing_pairs(include_str!("../day04"))
    );
    println!(
        "In how many assignment pairs do the ranges overlap? {}",
        count_overlaps_in_pairs(include_str!("../day04"))
    );
}
