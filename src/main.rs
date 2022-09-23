use std::cmp;

fn main() {
    mult_div_error("9.9999", 2, "0.1000" "9.99999", 4, "0.00011", "*");
}

fn mult_div_error(number1: &str, ten_power1: i32, error1: &str, number2: &str, ten_power2: i32, error2: &str, operation: &str){
    //make sure that the errors properly match the measurement and the number of decimal places
    if (&number1[(number1.to_string()).find(".")..]).len() != 
    
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

    println!("Num1 {}   Num2 {}", f64_clean_num1, f64_clean_num2);
    //2.) perform the operation
    let mut result: f64 = 0.0;
    if operation == "*" {
        result = f64_clean_num1 * f64_clean_num2;
    } else if operation == "/" {
        result = f64_clean_num1 / f64_clean_num2;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = cmp::min(num_places1, num_places2) as i32; //works
    println!("Result {}     Number of sigfigs to round to {}", result, least_sigfigs);


    let mut str_result: String = result.to_string();

    //3.) round to the appropriate number of sigfigs
    // First: obtain the digit after the last place
    //vectorize the digit to make this possible
    //Second: identify if this digit is >= 5, if so, then increase the previous digit by one

    // vectorize(str_result, &mut msmt_vector);
    let mut final_ten_power: i32 = 0;
    match operation {
        "*" => {
            final_ten_power = ten_power1 + ten_power2;
        }, 
        "/" => {
            final_ten_power = ten_power1 - ten_power2;
        },
        _ => panic!("Please enter either * for multiplication or / for division"),
    }

    let final_msmt: String = round_vector(&mut str_result, least_sigfigs, &mut msmt_vector, &mut final_ten_power);
    println!("{}E{}", final_msmt, final_ten_power);


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

fn vectorize(number: &mut String, msmt_vector: &mut Vec<u32>) { //this is for addition and subtraction
    
    for (idx, value) in number.chars().enumerate() {
        // msmt_vector.push(value.to_digit(10).unwrap());
        if value != '.' {
            msmt_vector.push(value.to_digit(10).unwrap());
        }
    }
}

fn print_number(msmt_vector: & Vec<u32>) {
    // for digit in msmt_vector {
    //     println!("{}", digit);
    // }
    println!("{:?}", msmt_vector);
}

fn round_vector(str_result: &mut String, least_sigfigs: i32, msmt_vector: &mut Vec<u32>, final_ten_power: &mut i32) -> String {
    let mut result: String = String::new();
    let mut idx = 0;
    let mut dec_pos: usize = str_result.find('.').unwrap(); //getting the position of the '.'
    //taken from the string; means that this can either be 1 or 2; must check if the first element in the vector regroups to add one more to the position of the '.'
    //round the number

    //1.) first get the vector without the decimal point
    vectorize(str_result, msmt_vector);
    let mut idx = 0;
    let mut regroup = false;
    let mut digit: u32 = msmt_vector[(least_sigfigs - idx) as usize]; //this starts at the digit after the last sigfig digit
    if digit >= 5 { //if the digit after the last sigfig is greater than or equal to 5, then the previous...
        regroup = true;
    }
    loop {
        let mut last_sigfig = msmt_vector[(least_sigfigs - 1 - idx) as usize];
        if regroup {
            last_sigfig += 1;
            if last_sigfig >= 10  {
                last_sigfig -= 10;
                regroup = true;
                if (1 + idx) == least_sigfigs { //this means that you regrouped to the first sigfig; you have to remove the last sigfig
                    result = format!("{}{}{}", (1).to_string(), last_sigfig, result);
                    result = (&result[0..result.len() - 1]).to_string();
                    dec_pos = dec_pos + (1 as u32 as usize)
                }
            } else {
                regroup = false;
            }
        } 

        result = format!("{}{}", last_sigfig, result);
        idx += 1;

        if (idx + 1) > least_sigfigs {
            break;
        }
    }

    result = format!("{}{}{}", &result[0..=0], ".".to_string(), &result[1..result.len()]);
    match dec_pos {
        2 => {*final_ten_power += 1},
        3 => {*final_ten_power += 2},
        _ => panic!("wtf did you do with the numbers"),
    }
    return result
}

