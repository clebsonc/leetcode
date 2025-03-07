use std::collections::HashMap;

fn get_mapped_numbers() -> HashMap<char, i32> {
    let numbers = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    numbers
}

pub fn roman_to_int(s: String) -> i32 {
    // number that can be seem togeter:
    // iv and ix = 4, 9
    // xl and xc = 40, 90
    // cd and cm = 400, 900
    //  MCMXCIV
    let chars = s.chars().collect::<Vec<char>>();
    let mut values_to_sum = Vec::<i32>::new();

    let mut index = 0;
    while index < chars.len() {
        let current_char = chars[index];
        let next_index = chars.get(index + 1);

        // if there is a next char.
        if let Some(next_char) = next_index {
            if current_char == 'I' && (*next_char == 'V' || *next_char == 'X') {
                let val = compute_subtraction(&current_char, &next_char);
                values_to_sum.push(val);
                index = index + 2;
                continue;
            } else if current_char == 'X' && (*next_char == 'L' || *next_char == 'C') {
                let val = compute_subtraction(&current_char, &next_char);
                values_to_sum.push(val);
                index = index + 2;
                continue;
            } else if current_char == 'C' && (*next_char == 'D' || *next_char == 'M') {
                let val = compute_subtraction(&current_char, &next_char);
                values_to_sum.push(val);
                index = index + 2;
                continue;
            }
        }

        let numbers = get_mapped_numbers();
        let val = numbers[&current_char];
        values_to_sum.push(val);
        index = index + 1;
    }
    let mut sum = 0;
    for x in &values_to_sum {
        sum += x;
    }
    return sum;
}

fn compute_subtraction(current_char: &char, next_char: &char) -> i32 {
    let numbers = get_mapped_numbers();
    let to_subtract = *numbers.get(&current_char).unwrap();
    let value = *numbers.get(&next_char).unwrap();
    value - to_subtract
}
