pub fn is_armstrong_number(num: u32) -> bool {

    let num_as_string = num.to_string();

    let num_of_digits: u32 = num_as_string.len() as u32;
    let v: u32 = num_as_string.chars().map(|c|c.to_string().parse::<u32>().unwrap().pow(num_of_digits)).sum();
    v == num
}
