use std::cmp;
fn main() {
    // mult_div_error("1.1", 1, "1.", "3.10", 0, "0.01", "*");
    // mult_div_error("7.6", 0, "0.4", "8.2", 0, "0.2", "*");
    // mult_div_error("8.", 0, "1.", "9.", 0, "2.", "*");
    // mult_div_error("2.6", 1, "3.", "7.", 0, "1.", "/");
    // mult_div_error("5.", 0, "1.", "7.", 0, "2.", "*");
    // add_sub_error("1.87", 1, "1.9", "2.1", 0, "0.1", "+");
    // add_sub_error("1.87", 1, "1.9", "2.1", 0, "0.1", "+");

    // mult_div_error("2.08", 1, "1.9", "2.1", 2, "2.", "/");
    // mult_div_error("2.08", 1, "1.9", "2.10", 2, "2.", "/");
    // mult_div_error("2.080", 0, "0.190", "2.100", 3, "2.", "/");

    // mult_div_error("2.08", 1, "1.9", "2.10", 2, "2.", "/");
    // mult_div_error("2.08", 1, "1.9", "2.1", 2, "2.", "/");
    // mult_div_error("2.08", 1, "1.9", "2.10", 2, "2.", "/");
    // mult_div_error("2.08", 0, "0.19", "2.1", 3, "2.", "/");

    add_sub_error("8.575", -1, "0.0429", "1.59", -1, "0.021", "-");
    mult_div_error("2.194", 0, "0.151", "1.1", -1, "0.05", "*");
    add_sub_error("9.0089", -1, "0.01802", "1.554", -1, "0.0016", "-");


}

fn add_sub_error(number1: &str, ten_power1: i32, error1: &str, number2: &str, ten_power2: i32, error2: &str, operation: &str) {
    let valid_input1: bool = check_error(&number1, &error1, &ten_power1);
    let valid_input2: bool = check_error(&number2, &error2, &ten_power2);
    if !valid_input1 || !valid_input2 {
        panic!("wtf are you doing with the msmts and errors");
    }

    let mut number1_vec: Vec<u32> = Vec::new();
    vectorize(& String::from(number1), &mut number1_vec);

    let mut number2_vec: Vec<u32> = Vec::new();
    vectorize(& String::from(number2), &mut number2_vec);

    //adding and manipulating 0s
    let mut dec_idx1: i32 = 0;
    let mut dec_idx2: i32 = 0;

    let mut m_ten_power1: i32 = ten_power1.clone();
    let mut m_ten_power2: i32 = ten_power2.clone();

    if ten_power1 != ten_power2 {
        if ten_power1 > ten_power2 { //ten_power1 is greater, so shift it down; ten_power2 is smaller so shift it up
            adjust_to_same_ten_power(&mut number1_vec, &mut m_ten_power1, &mut dec_idx1, &mut number2_vec, &mut m_ten_power2, &mut dec_idx2);
        } else if ten_power1 < ten_power2 { //ten_power1 is smaller, so shift it up; ten_power2 is bigger so shift it down
            adjust_to_same_ten_power(&mut number2_vec, &mut m_ten_power2, &mut dec_idx2, &mut number1_vec, &mut m_ten_power1, &mut dec_idx1);
        }
    }

    //get the same digits but make sure to keep the decimal place in the correct position; add 0s to match the place values but not change the value
    let most_num_whole_places = cmp::max(dec_idx1, dec_idx2);
    let num_dec_places1 = number2_vec.len() as i32 - dec_idx2 as i32 - 1_i32 - ten_power1;
    let num_dec_places2 = number1_vec.len() as i32 - dec_idx1 as i32 - 1_i32 - ten_power2;
    let most_num_dec_places = *cmp::max(& num_dec_places1, & num_dec_places2) as u32;
    
    check_whole_places(&mut number1_vec, &mut dec_idx1, most_num_whole_places);
    check_whole_places(&mut number2_vec, &mut dec_idx2, most_num_whole_places);
    check_dec_places(&mut number1_vec, dec_idx1, most_num_dec_places);
    check_dec_places(&mut number2_vec, dec_idx2, most_num_dec_places);
    //at this point, dec_idx1 and dec_idx2 are the same number


    // print_number(&number1_vec);
    // print_number(&number2_vec);

    let mut result_vec: Vec<u32> = Vec::new();
    
    let mut big_regroup = false; //for regrouping when i = 0; (the first sigfig)
    let mut final_ten_power = m_ten_power1.clone();
    match operation {
        "+" => {
            let mut regroup = false;
            for i in (0..number1_vec.len()).rev() {
                let mut digit = number1_vec[i] + number2_vec[i];
                if regroup {
                    digit += 1;
                    regroup = false;
                }

                if digit >= 10 {
                    digit -= 10;
                    regroup = true;
                    if i == 0 {
                        big_regroup = true;
                    }
                }
                result_vec.insert(0, digit as u32);
                
            }
            if big_regroup {
                result_vec.insert(0, 1_u32);
                dec_idx1 += 1;
                dec_idx2 += 1;
                final_ten_power += 1;
            }

        },
        "-" => {
            let mut regroup = false;
            for i in (0..number1_vec.len()).rev() {
                // let mut digit: i32 = number1_vec[i] as i32 - number2_vec[i] as i32;
                let mut num1 = number1_vec[i] as i32;
                // print_number(&number1_vec);
                // print_number(&number2_vec);
                let num2 = number2_vec[i] as i32;
                if regroup {
                    num1 -= 1;
                    if num1 == 0 {
                        final_ten_power -= 1;
                    }
                }

                if num1 < num2 {
                    regroup = true;
                    num1 += 10;
                }

                let digit = num1 - num2;

                result_vec.insert(0, digit as u32);
                
            }
        },
        _ => panic!("wtf are you doing, pick either + or -"),
    }

    let mut is_number = false;
    let mut result_str = String::new();

    let least_dec_places = cmp::min(num_dec_places1, num_dec_places2);
    // let mut starting_dec_places = false;
    // let mut num_dec_places_added = 0;
    //keep as whole number first and then round as integer and then put decimal place

    let mut final_regroup = false; //regroup or not
    // println!("{:?}", result_vec);
    for (idx, digit) in result_vec.iter().enumerate().rev() {
        let mut cur_digit = digit.clone();
        if is_number {
            if dec_idx1 == idx as i32 {
                if dec_idx1 != 0 {
                    dec_idx1 -= 1;
                    final_ten_power += 1;
                    if final_regroup {
                        cur_digit += 1;
                    }
                    if cur_digit >= 10 {
                        final_regroup = true;
                        cur_digit -= 10;
                    }
                    // println!("{}  {}", cur_digit, least_dec_places);
                    result_str = format!("{}{}", cur_digit, result_str);
                } else {
                    while result_vec[dec_idx1 as usize] == 0 {
                        dec_idx1 += 1;
                        final_ten_power -= 1;
                    }
                    let whole_part = result_vec[dec_idx1 as usize].to_string();
                    let dec_part = &result_str[0..];
                    result_str = format!("{}{}{}", whole_part, ".", dec_part);

                }
            } else {
                if final_regroup {
                    cur_digit += 1;
                    final_regroup = false;
                }
                if cur_digit >= 10 {
                    final_regroup = true;
                    cur_digit -= 10;
                }
                println!("{}  {}", cur_digit, least_dec_places);
                result_str = format!("{}{}", cur_digit, result_str);

            }
        }
        
        if idx == (dec_idx1 + least_dec_places + 1 + final_ten_power) as usize && !is_number { //get the digit after the last sigfig
            if *digit >= 5 {
                final_regroup = true;
            }
            is_number = true;
            
        }



        // result_str = format!("{}{}", cur_digit, result_str);


        // if !is_number && *digit != 0_u32 {
        //     is_number = true;
        //     result_str.push_str(&digit.to_string());
        //     if dec_idx1 == 0 {
        //         result_str.push_str(".");
        //         starting_dec_places = true;
        //     }
            
        //     // println!("{}   {}    {}", result_str, least_dec_places, dec_idx1);
        // } else if idx == dec_idx1 as usize && is_number && !starting_dec_places {
        //     result_str.push_str(&digit.to_string());
        //     starting_dec_places = true;
        // } else if is_number && starting_dec_places && num_dec_places_added < least_dec_places {
        //     num_dec_places_added += 1;
        //     result_str.push_str(&digit.to_string());
        // }


    }
    
    let result_num = result_str.parse::<f64>().unwrap();
    // *************
    
    let error: f64 = (error1.parse::<f64>().unwrap().powi(2) + error2.parse::<f64>().unwrap().powi(2)).sqrt();
    let mut error_str = error.to_string();
    error_str = error_str[0..=error_str.find(".").unwrap() + (least_dec_places + 1) as usize].to_string();
    
    let mut error_vec = Vec::new();
    vectorize(&error_str, &mut error_vec);
    let mut error_str2 = String::new();
    let mut final_error_str = String::new();
    if error_vec[error_vec.len() - 1] >= 5 {
        let mut regroup = true;
        for (idx, digit) in error_vec.iter().enumerate().rev() {
            let mut cur_digit = digit.clone();
            if idx != error_vec.len() - 1 {
                if regroup {
                    cur_digit += 1;
                    regroup = false;
                    if cur_digit >= 10 {
                        cur_digit -= 10;
                        regroup = true;
                    }
                }
                error_str2 = format!("{}{}", cur_digit, error_str2);

            }   
        }

        let whole_part = &error_str2[0..error_str2.len() - least_dec_places as usize];
        let dec_part = &error_str2[error_str2.len() - least_dec_places as usize..]; 
        
        final_error_str.push_str(whole_part);
        final_error_str.push('.');
        final_error_str.push_str(dec_part);
    } else {
        final_error_str = error_str[0..error_str.len() -2].to_string();
    }
    println!("{} E {}   error: {}", result_str, final_ten_power, final_error_str);
    

}


fn check_whole_places(number_vec: &mut Vec<u32>, dec_idx: &mut i32, num_whole_places: i32) {
    loop {
        
        if *dec_idx + 1 != num_whole_places {
            break;
        }

        number_vec.insert(0, 0);
        *dec_idx += 1;
    }
}

fn check_dec_places(number_vec: &mut Vec<u32>, dec_idx: i32, num_dec_places: u32) {
    while number_vec.len() as u32 - (dec_idx + 1) as u32 != num_dec_places{
        number_vec.push(0);
    }
}




fn adjust_to_same_ten_power(larger_vec: &mut Vec<u32>, larger_ten_power: &mut i32, larger_dec_idx: &mut i32, smaller_vec: &mut Vec<u32>, smaller_ten_power: &mut i32, smaller_dec_idx: &mut i32) {
    loop {
        if larger_ten_power != smaller_ten_power {
            //shift the larger_ten_power down, meaning add 0s at the beginning
            //shift the smaller_ten_power up, meaning add 1s at the beginning
            *larger_ten_power -= 1;
            *larger_dec_idx += 1;
            if *larger_dec_idx as u32 > larger_vec.len() as u32 {
                larger_vec.push(0);
            }

            if larger_ten_power != smaller_ten_power {
                *smaller_ten_power += 1;
                // smaller_dec_idx doesn't change
                smaller_vec.insert(0, 0);
            } else {
                break;
            }

        }
    }

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
    

    //2.) perform the operation
    let mut result: f64 = 0.0;
    if operation == "*" {
        result = f64_clean_num1 * f64_clean_num2;
    } else if operation == "/" {
        result = f64_clean_num1 / f64_clean_num2;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = cmp::min(num_places1, num_places2) as i32; //works
    
    let mut final_ten_power: i32 = 0;
    
    loop {
        if result < 1.0 {
            result *= 10.0;
            final_ten_power -= 1;
        } else {
            break;
        }
    }
        


    
    let mut str_result: String = result.to_string();
    if str_result.find('.') == None {
        str_result.push('.');
    }
    

    //3.) round to the appropriate number of sigfigs
    // First: obtain the digit after the last place
    //vectorize the digit to make this possible
    //Second: identify if this digit is >= 5, if so, then increase the previous digit by one

    // vectorize(str_result, &mut msmt_vector);
    match operation {
        "*" => {
            final_ten_power += ten_power1 + ten_power2;
        }, 
        "/" => {
            final_ten_power += ten_power1 - ten_power2;
        },
        _ => panic!("Please enter either * for multiplication or / for division"),
    }
    
    if &str_result[0..1] == "0" && final_ten_power > 0{
        str_result = (result * 10_f64.powi(final_ten_power)).to_string();
        final_ten_power = 0;
    }
    
    
    //clean up str_result so that it has a digit larger than 1

    let final_msmt: String = round_vector(&mut str_result, least_sigfigs, &mut msmt_vector, &mut final_ten_power);
    
    let mut final_msmt_num: f64 = final_msmt.parse::<f64>().unwrap();




    // *******************************************
    //Calculating the errors
    //find the percentages
    let e_percent1: f64 = error1.parse::<f64>().unwrap() / f64_clean_num1 / 10_f64.powi(ten_power1);
    let e_percent2: f64 = error2.parse::<f64>().unwrap() / f64_clean_num2 / 10_f64.powi(ten_power2);

    let e_result: f64 = f64::sqrt(e_percent1.powi(2) + e_percent2.powi(2)); //this is the RMS_percent
    
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
                let mut hundredths = e_vec[2];
                let mut tenths = e_vec[1];
                if e_vec[3] >= 5 {

                    hundredths += 1;
                    if hundredths >= 10 {
                        hundredths -= 10;
                        tenths += 1;
                    }
                }
                rms_percent_str = format!("{}{}{}", "0.", tenths, hundredths);
                break;
            } else { //means caught beyond the hundredths place
                let mut error_digit = *digit;
                if e_vec[idx + 1] >= 5 {
                    error_digit += 1;
                } 
                rms_percent_str = format!("{}{}", rms_percent_str, error_digit);
            }
            //if less than 1 percent, round to the first non-zero digit
            break;
        } else if idx != 0{
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
    let mut error_ten_power = 0;
    if final_msmt_num * ten.powi(final_ten_power) % 1.0 == 0.0 { //This means that the msmt is a whole number
        //The error when applied to just the msmt part, not the tens powers, could be a decimal
        //but it could also be a whole number
        
        if final_error_stage1 < 1.0 {
            //have to shift the decimal place until we get a whole number, number of places shifted subtract from the final-tens-power
            vectorize(& final_error_stage2, &mut final_error_vec);
            
            if final_msmt_num * rms_percent_f * 10_f64.powi(final_ten_power) < 1.0 {
                final_error_stage3 = String::from("1.");
            } else {
                decimal_to_sci_not(&mut final_error_stage3, &mut final_error_vec, &final_msmt, &final_ten_power, &mut error_ten_power);
            }

            
            

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
            
            if final_error_stage1 * 10_f64.powi(final_ten_power) % 1.0 == 0.0 {
                final_error_stage3 = final_error_stage2;
                error_ten_power = final_ten_power.clone();
            } else {
                vectorize(&final_error_stage2, &mut final_error_vec);
                final_error_stage3 = round_vector(&mut final_error_stage2, final_ten_power + 1, &mut final_error_vec, &mut error_ten_power);
                final_error_stage3 = final_error_stage3[0..final_error_stage2.find(".").unwrap() + (final_ten_power + 1) as usize].to_string();
                
                error_ten_power = final_ten_power.clone();
            }
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
            decimal_to_sci_not(&mut final_error_stage3, &mut final_error_vec, &final_msmt, &final_ten_power, &mut error_ten_power);
            
            
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
                final_error_stage3 = round_vector(&mut final_error_stage2, (dec_pos + num_dec_places - 1) as i32 , &mut final_error_vec, &mut error_ten_power);
            }
            // final_error_stage3.push_str(&final_error_stage2[0..dec_pos]);
            // final_error_stage3.push_str(&final_error_stage2[dec_pos..dec_pos + num_dec_places + 1]);
        }
    }
    println!("{} E {}   +- {} E {}", final_msmt, final_ten_power, final_error_stage3, error_ten_power);
    
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

    

    return temp_int
    

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
    println!("{:?}", msmt_vector);
}

fn round_vector(str_result: &mut String, least_sigfigs: i32, msmt_vector: &mut Vec<u32>, final_ten_power: &mut i32) -> String {
    //str_result is >= 1.0
    //rounds to the number of sigfigs
    //stores in msmt_vector
    //final_ten_power used for regrouping purposes
    // println!("{} {} {:?} {}", str_result, least_sigfigs, msmt_vector, final_ten_power);
    let mut result: String = String::new();
    

    let mut idx = 0;
    
    let mut dec_pos: usize = str_result.find('.').unwrap() - 1 as usize;
    
     //getting the position of the '.'
    //taken from the string; means that this can either be 1 or 2; must check if the first element in the vector regroups to add one more to the position of the '.'
    //round the number

    //1.) first get the vector without the decimal point
    if !msmt_vector.is_empty() {
        msmt_vector.clear();
    }

    vectorize(str_result, msmt_vector);
    

    
    let mut regroup = false;
    let mut extra_for_less_than_zero = 0;
    // if msmt_vector[0] == 0 {
    //     extra_for_less_than_zero = 1;
    // }

    if least_sigfigs == 0 {
        extra_for_less_than_zero = 1;
    }
    
    let digit: u32 = msmt_vector[(least_sigfigs + extra_for_less_than_zero) as usize]; //this starts at the digit after the last sigfig digit
    if digit >= 5 { //if the digit after the last sigfig is greater than or equal to 5, then the previous...
        regroup = true;
    }

    
    loop {
        let mut last_sigfig = msmt_vector[(least_sigfigs + extra_for_less_than_zero - 1 - idx) as usize];

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
        0 => (),
        1 => {*final_ten_power += 1},
        2 => {*final_ten_power += 2},
        _ => {
            panic!("wtf did you do with the numbers; Here is your result: {} The dec_pos was at: {}", result, dec_pos);
        }, 
    }
    return result
}

fn check_error(number: &str, error: &str, ten_power: &i32) -> bool {
    // if ((*ten_power as usize) < number[number.find(".").unwrap()..].len() - 1) && (number[(number.to_string()).find(".").unwrap()..]).len() != ( error[(error.to_string()).find(".").unwrap()..].len()) { //if the number has decimal places   
    //     return false
    // }
    // } else if error.find(".").unwrap() == (error.len() - 1) { //meaning whole measurement: so error no decimal points
    //     return false
    // }
    let mut n_dec_idx = number.find(".").unwrap();
    let mut n_dec_places = (number[n_dec_idx..].len() as u32 - 1_u32) as i32;
    let mut e_dec_idx = error.find(".").unwrap();
    let mut e_dec_places = (error[e_dec_idx..].len() as u32 - 1_u32) as i32;
    if n_dec_places < *ten_power{
        n_dec_places = 0;
    } else {
        n_dec_places -= *ten_power;
    }

    // println!("{} {}    {} {}",number, n_dec_places, error, e_dec_places);
    if e_dec_places != n_dec_places {
        return false
    }

    return true
}

//for errors only; considering that at this point, the final_ten_power will be, (not yet tho) the error_ten_power
fn decimal_to_sci_not(str_result: &mut String, final_error_vec: &mut Vec<u32>, final_msmt: &String, final_ten_power: & i32, error_ten_power: &mut i32) {
    //the purpose of this function is to turn a decimal less than 1 to scientific notation
    //str_result is empty: it is intended to hold the result
    //final_error_vec contains the number
    //error_ten_power is empty
    //Problem how to get which place to round to: final_ten_power.abs() tells you which idx the number starts at (1 * 10^-2 -> 0.0* where * is at idx 2 in vector)
    //then you just have to consider the final_msmt.len() - 1 - 1 (remember the decimal place and the ones place)
    let mut reduce_ten_pow: i32 = 0;
    let mut is_number: bool = false;
    let mut need_to_round = false;
    let mut num_digits_left: i32 = -1;
    // println!("{:?}  {}", final_error_vec, error_ten_power);
    //this is the part actually doing the conversion to sci
    let mut num_dec_places_in_msmt = (final_msmt[final_msmt.find(".").unwrap()..].len() as u32 - 1_u32) as i32 - *final_ten_power;

    for (idx, digit) in final_error_vec.iter().enumerate() {
        if *digit != 0 && is_number == false && idx != 0{
            reduce_ten_pow += 1;
            str_result.push_str(&digit.to_string());
            str_result.push('.');

            *error_ten_power += *final_ten_power;
            *error_ten_power -= reduce_ten_pow;
            // println!("{}", error_ten_power);
            
            //need the same number of decimal places as the final_msmt
            //how do you find the number of decimal places in the final_msmt
            //take the number of decimal places available first, subtract the tens powers from it (-tens powers add to it)
            // num_digits_left = *final_ten_power; //if your error was 0.0005 * 10^-6, for everytime you moved the decimal place up to make it larger, you have to decrease the power 
            // num_digits_left -= reduce_ten_pow; //we run into a problem when the result is 0
            // num_digits_left = num_digits_left.abs();
            // println!("{}", num_digits_left);
            
            if num_dec_places_in_msmt < 0 { // this means that the number has no decimal places
                num_dec_places_in_msmt = 0;
            } else { //otherwise if there are decimal places, we need to consider how many to grab; remember that the error_vec 
                //that was passed in is always < 0 and will have the final_ten_power applied
                num_digits_left =  num_dec_places_in_msmt as i32 - reduce_ten_pow + *final_ten_power;
                
            }
            // println!("{}    {}    {}", final_msmt, num_digits_left, num_dec_places_in_msmt);
            
            //else if *final_ten_power < 0 {
            //     *error_ten_power += ;
            // }
            //must also add the number of decimal places from the final_msmt
            // let final_msmt_dec_places = final_msmt[final_msmt.find(".").unwrap()..].len() as i32 - final_ten_power;
            // num_digits_left += 
            
            
            is_number = true;
            
        } else if idx != 0 && !is_number {
            reduce_ten_pow += 1;

        } else if is_number == true {
            if num_digits_left != 0 {
                str_result.push_str(&digit.to_string());
                num_digits_left -= 1;
            }

            if idx == final_error_vec.len() - 1 && num_digits_left != 0 {
                for i in [0..num_digits_left] {
                    str_result.push('0');
                }
            }

        }
        


        if num_digits_left == 0 {
            if idx != final_error_vec.len() { //meaning that there are still digits left, then you need to round
                str_result.push_str(&(final_error_vec[idx + 1]).to_string());
                need_to_round = true;
            } //otherwise, there's no need to round because you caught the last digit and there's nothing you need to look at to round
            
            break;
        }


    }

    // print_number(&final_error_vec);
    if need_to_round {
        // let mut num_dec_places_round = final_ten_power.abs() as i32 + (final_msmt.len() as i32 - 2); //this will get you how many decimal places the final_msmt has
        let mut num_dec_places_round = num_dec_places_in_msmt.clone();
        // if *final_ten_power > 0 {
        //     num_dec_places_round -= *final_ten_power
        // }
        
        num_dec_places_round += *error_ten_power + 1;
        
        // println!("{} {}", num_dec_places_in_msmt, num_dec_places_round);

        //error_ten_power is always negative

        //REMEMBER: final_ten_power = error_ten_power; BUT error_ten_power changes depending on how much smaller the error was, 
        //since error is smaller, sometimes the error_ten_power is a larger negative value; meaning less places for the error

        //you are rounding the error so this may lead to changes in the error_ten_power, not the final_ten_power of the msmt
        // println!("{}", num_dec_places_round);
        // print_number(&final_error_vec);
        *str_result = round_vector(str_result, num_dec_places_round, final_error_vec, error_ten_power);
    }
    //now final_error_stage3 will hold the final result, but you need to remember to round
}