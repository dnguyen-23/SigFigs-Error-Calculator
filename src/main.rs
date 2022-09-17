fn main() {
    
}

fn mult_div_error(number1: &str, ten_power1: i32, number2: &str, ten_power2: i32) {
    let mut msmt_vector: Vec<u32> = Vec::new();
    let mut isNegative1: bool = false;
    count_sig_figs(number1, &mut msmt_vector, &mut isNegative);
    print_number(msmt_vector);
    // count_sig_figs(number2, &mut msmt_vector, &mut isNegative);

}

fn print_number(msmt_vector: & Vec<u32>) {
    for digit in msmt_vector {
        println!("{}", digit);
    }
}

fn count_sig_figs(number: &str, msmt_vector: &mut Vec<u32>, isNegative: &mut bool) -> i32{
    
    let radix: i32 = 10;
    for (idx, value) in number.chars().enumerate() {
        match value {
            '-' => isNegative = !isNegative,
            _ => if value != '.' { msmt_vector.push((value.to_digit(10).unwrap()))}
        }
    }



    print_number(msmt_vector);

    num_sigfigs
}