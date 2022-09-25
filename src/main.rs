use std::cmp;

fn main() {
    mult_div_error("1.1", 1, "1.", "3.10", 0, "0.01", "*");
    mult_div_error("7.6", 0, "0.4", "8.2", 0, "0.2", "*");
    mult_div_error("8.", 0, "1.", "9.", 0, "2.", "*");
    mult_div_error("2.6", 1, "3.", "7.", 0, "1.", "/");
}

fn mult_div_error(number1: &str, ten_power1: i32, error1: &str, number2: &str, ten_power2: i32, error2: &str, operation: &str){
    //make sure that the errors properly match the measurement and the number of decimal places
    let valid_input1: bool = check_error(&number1, &error1, &ten_power1);
    let valid_input2: bool = check_error(&number2, &error2, &ten_power2);
    if !valid_input1 || !valid_input2 {
        panic!("wtf are you doing with the msmts and errors");
    }


    
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
    // println!("Num1 {}   Num2 {}", f64_clean_num1, f64_clean_num2);

    //2.) perform the operation
    let mut result: f64 = 0.0;
    if operation == "*" {
        result = f64_clean_num1 * f64_clean_num2;
    } else if operation == "/" {
        result = f64_clean_num1 / f64_clean_num2;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = cmp::min(num_places1, num_places2) as i32; //works
    // println!("Result {}     Number of sigfigs to round to {}", result, least_sigfigs);


    let mut str_result: String = result.to_string();
    if str_result.find('.') == None {
        str_result.push('.');
    }
    //3.) round to the appropriate number of sigfigs
    // First: obtain the digit after the last place
    //vectorize the digit to make this possible
    //Second: identify if this digit is >= 5, if so, then increase the previous digit by one

    // vectorize(str_result, &mut msmt_vector);
    let mut final_ten_power: i32;
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
    

    let mut final_msmt_num: f64 = final_msmt.parse::<f64>().unwrap();

    // *******************************************
    //Calculating the errors
    //find the percentages
    let e_percent1: f64 = error1.parse::<f64>().unwrap() / f64_clean_num1;
    let e_percent2: f64 = error2.parse::<f64>().unwrap() / f64_clean_num2;
    let e_result: f64 = f64::sqrt(e_percent1.powi(2) + e_percent2.powi(2));
    
    let mut e_str: String = e_result.to_string();
    e_str = e_str.replace(".", ""); //you always know that the percentage can never be over 100
    //this means that as a decimal, the value is never greater than 10 or even 1
    //there will alwayas be a ones place
    let mut e_vec: Vec<u32> = Vec::new();
    vectorize(& e_str, &mut e_vec);
    let mut rms_percent_str = "0.".to_string();
    for (idx, digit) in e_vec.iter().enumerate() {

        if *digit != 0 {
            //if larger than 1 percent, round to the whole percent; round to the nearest hundredths place
            if idx == 1 || idx == 2 { //means caught in the tenths or hundredths place, need to round to the nearest hundredths place
                let mut hundredths = e_vec[2] + 1;
                let mut tenths = e_vec[1];
                if e_vec[3] >= 5 {
                    if hundredths >= 10 {
                        hundredths -= 10;
                        tenths += 1;
                    }
                }
                rms_percent_str = format!("{}{}{}", rms_percent_str, tenths, hundredths);
            } else { //means caught beyond the hundredths place
                let mut error_digit = *digit;
                if e_vec[idx + 1] >= 5 {
                    error_digit += 1;
                } 
                rms_percent_str = format!("{}{}", rms_percent_str, error_digit);
            }
            //if less than 1 percent, round to the first non-zero digit
            break;
        } else {
            rms_percent_str.push('0');
        }
        
    }

    let rms_percent_f: f64 = rms_percent_str.parse::<f64>().unwrap();
    let final_error_stage1 =  final_msmt_num * rms_percent_f; //now you must consider the tens powers
    let mut ten: f64 = 10.0;
    let mut final_error_stage2: String = final_error_stage1.to_string();
    if final_error_stage2.find(".") == None {
        final_error_stage2.push('.');
    }
    let mut final_error_stage3: String = String::new();
    let mut final_error_vec: Vec<u32> = Vec::new();
    let mut error_ten_power: i32 = 0;
    if final_msmt_num * ten.powi(final_ten_power) % 1.0 == 0.0 { //This means that the msmt is a whole number
        //The error when applied to just the msmt part, not the tens powers, could be a decimal
        //but it could also be a whole number
        if final_error_stage1 < 1.0 {
            //have to shift the decimal place until we get a whole number, number of places shifted subtract from the final-tens-power
            vectorize(& final_error_stage2, &mut final_error_vec);
            decimal_to_sci_not(&mut final_error_stage3, &mut final_error_vec, &final_msmt, &final_ten_power, &mut error_ten_power);
            // final_error_stage3 will hold the final version of the error as a string
            // final_error_vec is used to help with manipulating the digits to get the number into scientific notation
            // error_ten_power holds the power that the error will be raised to
            // this method also accounts for rounding

            // let mut reduce_ten_pow: i32 = 0;
            // let mut is_number: bool = false;           
            // let mut num_digits_left: i32 = 0;
            // for (idx, digit) in final_error_vec.iter().enumerate() {
            //     if *digit != 0 && is_number == false{
            //         reduce_ten_pow += 1;
            //         final_error_stage3.push_str(&digit.to_string());
            //         final_error_stage3.push('.');
            //         error_ten_power = final_ten_power - reduce_ten_pow;
            //         num_digits_left = final_ten_power - reduce_ten_pow;          
            //         is_number = true;         
            //     } else if idx != 0 {
            //         reduce_ten_pow += 1;
            //     } else if is_number == true {
            //         final_error_stage3.push_str(&digit.to_string());
            //         num_digits_left -= 1;
            //         if num_digits_left == 0 {
            //             break;
            //         }
            //     }
            // }
            
        } else if final_error_stage1 > 1.0 {
            //have to shift the decimal place 
            final_error_stage3 = final_error_stage2[0..final_error_stage2.find(".").unwrap() + 1].to_string();
        }
        
    } else { //if there are decimal places, we need to check that there are the same number of decimal placles
        //find the number of decimal places in the final msmt
        //find what the tens_power is
        //Possible cases: 1.) the error has too many decimal places 
        //2.) the error needs more decimal places so add 0s
        //3.) the error has enough decimal places (will be determined when checking both)

        if final_error_stage1 < 1.0 { //error is a number smaller than 1
            // let num_dec_places: usize = final_ten_power * -1 + final_msmt.len() as u32 - 1;
            let dec_pos = final_error_stage2.find(".").unwrap() as u32;
            vectorize(&final_error_stage2, &mut final_error_vec);
            decimal_to_sci_not(&mut final_error_stage3, &mut final_error_vec, &final_msmt, &final_ten_power, &mut error_ten_power)

        } else if final_error_stage1 > 1.0 { //otherwise we have a number larger than 1 but with decimal places
            let num_dec_places: usize = final_msmt[final_msmt.find(".").unwrap()..final_msmt.len()].len();
            let dec_pos = final_error_stage2.find(".").unwrap();
            if dec_pos + num_dec_places == final_error_stage2.len() { //has the perfect amt
                final_error_stage3 = final_error_stage2;
            } else if dec_pos + num_dec_places > final_error_stage2.len() { //meaning that the needed dec_places is more; so add 0s
                let num_zeros = dec_pos + num_dec_places - (final_error_stage2.len());
                for i in [0..num_zeros] {
                    final_error_stage2.push('0');
                }
            } else if dec_pos + num_dec_places < final_error_stage2.len() { //meaning that the needed dec_places is less; so rounding
                error_ten_power = final_ten_power;
                round_vector(&mut final_error_stage2, (dec_pos + num_dec_places - 1) as i32 , &mut final_error_vec, &mut error_ten_power);
            }
            // final_error_stage3.push_str(&final_error_stage2[0..dec_pos]);
            // final_error_stage3.push_str(&final_error_stage2[dec_pos..dec_pos + num_dec_places + 1]);
        }
    }
    println!("{} E {}   +- {}", final_msmt, final_ten_power, final_error_stage3);
    
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

    // println!("{}", temp_int);

    return temp_int
    // print_number(&msmt_vector);
    // let num_sigfigs = msmt_vector.len() as i32;
    // let num_sigfigs = number.len() as i32;
    // println!("{}", num_sigfigs);

}

fn vectorize(number: &String, msmt_vector: &mut Vec<u32>) { //this is for addition and subtraction
    
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
    //str_result is >= 1.0
    //rounds to the number of sigfigs
    //stores in msmt_vector
    //final_ten_power used for regrouping purposes
    let mut result: String = String::new();
    let mut idx = 0;
    // println!("{}", str_result);
    let mut dec_pos: usize = str_result.find('.').unwrap();
    
     //getting the position of the '.'
    //taken from the string; means that this can either be 1 or 2; must check if the first element in the vector regroups to add one more to the position of the '.'
    //round the number

    //1.) first get the vector without the decimal point
    if !msmt_vector.is_empty() {
        msmt_vector.clear();
    }

    vectorize(str_result, msmt_vector);
    let mut regroup = false;
    let digit: u32 = msmt_vector[(least_sigfigs - idx) as usize]; //this starts at the digit after the last sigfig digit
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
        1 => (),
        2 => {*final_ten_power += 1},
        3 => {*final_ten_power += 2},
        _ => {
            panic!("wtf did you do with the numbers; Here is your result: {} The dec_pos was at: {}", result, dec_pos);
        }, 
    }
    return result
}

fn check_error(number: &str, error: &str, ten_power: &i32) -> bool {
    // println!("{:?}    {:?}", (number[(number.to_string()).find(".").unwrap()..]).len(), ( error[(error.to_string()).find(".").unwrap()..].len()));
    if ((*ten_power as usize) < number[number.find(".").unwrap()..].len() - 1) && (number[(number.to_string()).find(".").unwrap()..]).len() != ( error[(error.to_string()).find(".").unwrap()..].len()) { //if the number has decimal places   
        return false
    }
    // } else if error.find(".").unwrap() == (error.len() - 1) { //meaning whole measurement: so error no decimal points
    //     return false
    // }
    return true
}

fn decimal_to_sci_not(str_result: &mut String, final_error_vec: &mut Vec<u32>, final_msmt: &String, final_ten_power: & i32, error_ten_power: &mut i32) {
    //the purpose of this function is to turn a decimal less than 1 to scientific notation
    //str_result is empty: it is intended to hold the result
    //Problem how to get which place to round to: final_ten_power.abs() tells you which idx the number starts at (1 * 10^-2 -> 0.0* where * is at idx 2 in vector)
    //then you just have to consider the final_msmt.len() - 1 - 1 (remember the decimal place and the ones place)
    let mut reduce_ten_pow: i32 = 0;
    let mut is_number: bool = false;
    let mut need_to_round = false;
    let mut num_digits_left: i32 = 0;
    for (idx, digit) in final_error_vec.iter().enumerate() {
        if *digit != 0 && is_number == false{
            reduce_ten_pow += 1;
            str_result.push_str(&digit.to_string());
            str_result.push('.');
            *error_ten_power -= final_ten_power;
            *error_ten_power -= reduce_ten_pow;

            num_digits_left -= final_ten_power;
            num_digits_left -= reduce_ten_pow;
            
            is_number = true;
            
        } else if idx != 0 {
            reduce_ten_pow += 1;

        } else if is_number == true {
            str_result.push_str(&digit.to_string());
            num_digits_left -= 1;
            if num_digits_left == 0 {
                if idx != final_error_vec.len() { //meaning that there are still digits left, then you need to round
                    str_result.push_str(&(final_error_vec[idx + 1]).to_string());
                } //otherwise, there's no need to round because you caught the last digit and there's nothing you need to look at to round
                break;
            }
        }

    }

    if need_to_round {
        let mut num_dec_places_round = final_ten_power.abs() as i32 + (final_msmt.len() as i32 - 2); //this will get you how many decimal places the final_msmt has
        num_dec_places_round -= error_ten_power.abs() as i32; 
        //REMEMBER: final_ten_power = error_ten_power; BUT error_ten_power changes depending on how much smaller the error was, 
        //since error is smaller, sometimes the error_ten_power is a larger negative value; meaning less places for the error

        //you are rounding the error so this may lead to changes in the error_ten_power, not the final_ten_power of the msmt
        round_vector(str_result, num_dec_places_round, final_error_vec, error_ten_power);
    }
    //now final_error_stage3 will hold the final result, but you need to remember to round
}