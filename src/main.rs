use std::cmp;

fn main() {
    mult_div_error("1234.12341234", 2, "-1234.1234", 4, "*");
    // println!("{}", 1.27348509 * 9.12346789)
}

fn mult_div_error(number1: &str, ten_power1: i32, number2: &str, ten_power2: i32, operation: &str){
    let mut msmt_vector: Vec<u32> = Vec::new();
    let mut is_negative: bool = false; //tells whether the result will be negative or not
    
    //1.) remove the negative sign
    let clean_num1: String = String::from(clean_num(&number1, &mut is_negative, false));
    let mut num_places1: i32 = clean_num1.len() as i32; //for multiplication
    if clean_num1.find(".") != None {
        num_places1 -= 1;

    }
    let f64_clean_num1: f64 = clean_num1.parse::<f64>().unwrap();

    let clean_num2: String = String::from(clean_num(&number2, &mut is_negative, false));
    let mut num_places2: i32 = clean_num2.len() as i32; //for multiplication
    if clean_num2.find(".") != None {
        num_places2 -= 1;
    }
    let f64_clean_num2: f64 = clean_num2.parse::<f64>().unwrap();

    //2.) perform the operation
    let mut result: f64 = 0.0;
    if operation == "*" {
        result = f64_clean_num1 * f64_clean_num2;
    } else if operation == "/" {
        result = f64_clean_num1 / f64_clean_num2;
    }

    let mut str_result: String = String::from(result.to_string());

    let least_sigfigs = cmp::min(num_places1, num_places2) as i32;
    println!("{}", least_sigfigs);

    //3.) round to the appropriate number of sigfigs
    let mut dec_idx = str_result.find(".").unwrap();
    if dec_idx != 1 { //this means that the result is >= 10; this means move the decimal point back and add 
        let mut do_round: bool = false;
        if str_result[least_sigfigs as u32].to_digit(10).unwrap() >= 5 {
            do_round = true;
        } 
        str_result = str_result.replace(".", "").substring(0, (least_sigfigs as usize)); //str_result no longer has a decimal point
        let mut round_result: i64 = str_result.parse::<i64>().unwrap();
        if do_round {
            round_result += 1;
        }
        str_result.insert(1, '.');
    }

    // print_number(&msmt_vector);
    // count_sig_figs(number2, &mut msmt_vector, &mut isNegative);

}

//This function removes the decimal point and the negative sign;
fn clean_num<'a>(number: &'a str, is_negative: &'a mut bool, is_adding: bool) -> String{
    let mut temp_int = String::from(number); //the number string; in case this is just a whole number
    if is_adding == true {
        if number.find(".") != None {
            temp_int = number.replace(".", "");
        }
    } else {
        if number.find("-") != None {
            temp_int = number.replace("-", "");
            //switch the negative result status
            if *is_negative == true {
                *is_negative = false;
            } else {
                *is_negative = true;
            }
        }

    }

    println!("{}", temp_int);

    return temp_int
    // print_number(&msmt_vector);
    // let num_sigfigs = msmt_vector.len() as i32;
    // let num_sigfigs = number.len() as i32;
    // println!("{}", num_sigfigs);

}

fn vectorize(number: &str, msmt_vector: &mut Vec<u32>) { //this is for addition and subtraction
    for (idx, value) in number.chars().enumerate() {
        msmt_vector.push(value.to_digit(10).unwrap());
    }
}

fn print_number(msmt_vector: & Vec<u32>) {
    // for digit in msmt_vector {
    //     println!("{}", digit);
    // }
    println!("{:?}", msmt_vector);
}