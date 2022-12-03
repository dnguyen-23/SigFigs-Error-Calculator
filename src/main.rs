use core::time;
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
    // mult_div_error("2.194", 0, "0.151", "1.1", -1, "0.05", "*");

    // add_sub_error("8.575", -1, "0.0429", "1.59", -1, "0.021", "-");
    // mult_div_error("2.194", 0, "0.151", "1.1", -1, "0.05", "*");
    
    //density
    // add_sub_error("8.575", -1, "0.0429", "1.59", -1, "0.021", "-");
    // mult_div_error("2.20", 0, "0.15", "1.1", -1, "0.05", "*");
    
    // add_sub_error("9.0089", -1, "0.01802", "1.554", -1, "0.0016", "-");
    // mult_div_error("2.342", 0, "0.057","1.25", -1, "0.007", "*");

    // add_sub_error("9.29007", -1, "0.037160", "1.554", -1, "0.0016", "-");
    // mult_div_error("2.430", 0, "0.117", "1.6713", -1, "0.00881", "*");
    
    // println!("\n\n\n\n");
    // mult_div_error("2.508", 0, "0.034", "2.4", -1, "0.11", "/");
    // mult_div_error("2.508", 0, "0.034", "2.93", -1, "0.018", "/");
    // mult_div_error("2.508", 0, "0.034", "4.061", -1, "0.0284", "/");
    // mult_div_error("8.450", 0, "0.015", "1.0718", 0, "0.0006", "/");

    // mult_div_error("4.000", 1, "0.05", "2.32935", -1, "0.002329", "/");
    // mult_div_error("5.000", 1, "0.05", "2.96698", -1, "0.002967", "/");
    // mult_div_error("6.000", 1, "0.05", "3.75361", -1, "0.003754", "/");
    // mult_div_error("7.000", 1, "0.05", "6.45067", -1, "0.001450", "/");
    // mult_div_error("8.000", 1, "0.05", "4.77388", -1, "0.004774", "/");
    // println!("\n");
    // mult_div_error("4.000", 1, "0.05", "1.51113", -1, "0.004533", "/");
    // mult_div_error("5.000", 1, "0.05", "1.87143", -1, "0.007486", "/");
    // mult_div_error("6.000", 1, "0.05", "2.20743", -1, "0.006622", "/");
    // mult_div_error("7.000", 1, "0.05", "2.48469", -1, "0.002485", "/");
    // mult_div_error("8.000", 1, "0.05", "2.78010", -1, "0.002224", "/");

    
    // mult_div_error("1.9051", -2, "0.000005", "1.607", -2, "0.00015", "/");
    // mult_div_error("1.9051", -2, "0.000005", "1.607", -2, "0.00015", "/");

    // free fall lab
    let msmts_neg2 = vec![["1.607", "0.00015"],
                                        ["1.617", "0.00009"],
                                        ["1.597", "0.00007"],
                                        ["1.610", "0.00006"],
                                        ["1.603", "0.00013"],
                                        ["1.600", "0.00006"],
                                        ["1.607", "0.00003"],
                                        ["1.603", "0.00009"]];

    for idx in msmts_neg2.iter() {
        mult_div_error("1.9051", -2, "0.000005", idx[0], -2, idx[1], "/", false);
    }

    println!("\n\n\n\n");

    let msmts_neg3 = vec![["8.767", "0.000176"],
                                        ["8.033", "0.000120"],
                                        ["7.033", "0.000088"],
                                        ["6.833", "0.000167"],
                                        ["6.333", "0.000176"],
                                        ["6.067", "0.000133"],
                                        ["5.700", "0.000100"],
                                        ["5.500", "0.000115"]];

    for idx in msmts_neg3.iter() {
        mult_div_error("1.9051", -2, "0.000005", idx[0], -3, idx[1], "/", false);
    }

    println!("\n\n\n");
    // add_sub_error("2.17", 0, "0.04", "1.187", 0, "0.011", "-");
    let msmts_vfinal = vec![["2.173", "0.043"],
                                    ["2.372", "0.024"],
                                    ["2.709", "0.027"],
                                    ["2.788", "0.056"],
                                    ["3.008", "0.090"],
                                    ["3.140", "0.063"],
                                    ["3.342", "0.067"],
                                    ["3.464", "0.069"]];

    let mut diff: Vec<[String; 3]> = Vec::new();
    let time_betw: Vec<[&str; 3]> = vec![["9.5500", "-2", "0.000058"],
                                        ["1.1893", "-1", "0.00007"],
                                        ["1.3963", "-1", "0.00003"],
                                        ["1.5907", "-1", "0.00019"],
                                        ["1.7740", "-1", "0.00031"],
                                        ["1.9390", "-1", "0.00006"],
                                        ["2.1060", "-1", "0.00021"],
                                        ["2.2543", "-1", "0.00028"]];
    for (idx, velocity) in msmts_vfinal.iter().enumerate() {
        let cur_diff: Vec<String> = add_sub_error(velocity[0], 0, velocity[1], "1.187", 0, "0.011", "-", true);
        
        // print!("difference: {} E {}  time:  {} E {}   ", cur_diff[0], cur_diff[1], time_betw[idx][0], time_betw[idx][1]);
        mult_div_error(cur_diff[0].as_str(), cur_diff[1].parse::<i32>().unwrap(), cur_diff[2].as_str(), time_betw[idx][0], time_betw[idx][1].parse::<i32>().unwrap(), time_betw[idx][2], "/", false);

    }


    let distance: Vec<[&str; 3]> = vec![["1.500", "-1", "0.0005"],
                                        ["2.000", "-1", "0.0005"],
                                        ["2.500", "-1", "0.0005"],
                                        ["3.000", "-1", "0.0005"],
                                        ["3.500", "-1", "0.0005"],
                                        ["4.000", "-1", "0.0005"],
                                        ["4.500", "-1", "0.0005"],
                                        ["5.000", "-1", "0.0005"]];


    println!("\n\n\n\nexponential error");
    for (idx, time) in time_betw.iter().enumerate() {
        println!("Distance {}", idx + 1);
        
        let calc1: Vec<String> = mult_div_error("1.187", 0, "0.011", time[0], time[1].parse::<i32>().unwrap(), time[2], "*", false);
        let calc2: Vec<String> = add_sub_error(distance[idx][0], distance[idx][1].parse::<i32>().unwrap(), distance[idx][2], calc1[0].as_str(), calc1[1].parse::<i32>().unwrap(), calc1[2].as_str(), "-", false);
        let calc3: Vec<String> = constant_mult_err(calc2[0].as_str(), calc2[1].parse::<i32>().unwrap(), calc2[2].as_str(), 2, false);
        let time_squared =  exp_error(time[0], time[1].parse::<i32>().unwrap(), time[2], 2, false);
        mult_div_error(calc3[0].as_str(), calc3[1].parse::<i32>().unwrap(), calc3[2].as_str(), time_squared[0].as_str(), time_squared[1].parse::<i32>().unwrap(), time_squared[2].as_str(), "/", false);
        
        println!("\n\n\n");
    }
    



}

fn add_sub_error(number1: &str, ten_power1: i32, error1: &str, number2: &str, ten_power2: i32, error2: &str, operation: &str, no_print: bool) -> Vec<String> {
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
    let dec_idx1: i32 = 0;
    let dec_idx2: i32 = 0;

   
    let mut num_dec_places1 = number1_vec.len() as i32 - dec_idx2 as i32 - 1_i32 - ten_power1;
    let mut num_dec_places2 = number2_vec.len() as i32 - dec_idx1 as i32 - 1_i32 - ten_power2;
    if num_dec_places1 < 0 {
        num_dec_places1 = 0;
    }
    if num_dec_places2 < 0 {
        num_dec_places2 = 0;
    }


    let mut result: f32 = 0.0;
    if operation == "-" {
        result = &number1.parse::<f32>().unwrap() * 10f32.powf(ten_power1 as f32) - &number2.parse::<f32>().unwrap() * 10f32.powf(ten_power2 as f32);
    } else if operation == "+" {
        result = &number1.parse::<f32>().unwrap() * 10f32.powf(ten_power1 as f32) + &number2.parse::<f32>().unwrap() * 10f32.powf(ten_power2 as f32);
    }


    let mut result_ten_power: i32 = 0;
    if result < 10_f32 {
        while result < 1.0 {
            result *= 10_f32;
            result_ten_power -= 1;
        }
    } else if result > 10_f32 {
        while result > 10.0 {
            result /= 10_f32;
            result_ten_power += 1;
        }
    }


    let mut result_str = String::new();

    let actual_least_dec_places = cmp::min(num_dec_places1, num_dec_places2); //will be used for the error
    let least_dec_places = &actual_least_dec_places + &result_ten_power;    // let most_num_dec_places = *cmp::max(& num_dec_places1, & num_dec_places2) as u32;
    


    result_str = format!("{:.dec_places$}", result, dec_places = least_dec_places as usize);
    // println!("Result String: {}    Least number of decimal places: {}    Numerical Result: {}", result_str, least_dec_places, result);
    let mut error: f32 = (error1.parse::<f32>().unwrap().powf(2.0) + error2.parse::<f32>().unwrap().powf(2.0)).powf(0.5);
    let str_error: String = format!("{:.dec_places$}", error, dec_places = actual_least_dec_places as usize);

    if !no_print {
        println!("{} E {}   {}", result_str, result_ten_power, str_error);
    }
    let result_vec: Vec<String> = vec![result_str, result_ten_power.to_string(), str_error];
    return result_vec

}




fn mult_div_error(number1: &str, ten_power1: i32, error1: &str, number2: &str, ten_power2: i32, error2: &str, operation: &str, no_print: bool) -> Vec<String>{
    //make sure that the errors properly match the measurement and the number of decimal places
    let valid_input1: bool = check_error(&number1, &error1, &ten_power1);
    let valid_input2: bool = check_error(&number2, &error2, &ten_power2);
    if !valid_input1 || !valid_input2 {
        panic!("wtf are you doing with the msmts and errors");
    }

    // 1.) take care of the negative sign; take care of the number of sig figs for each number
    let mut is_negative: bool = false;
    let num1: f64 = number1.parse::<f64>().unwrap() * 10_f64.powf(ten_power1 as f64);
    let mut num_places1 = number1.len();
    if number1.contains('.') {
        num_places1 -= 1;
    }
    if number1.contains('-') {
        num_places1 -= 1;
    }

    let num2: f64 = number2.parse::<f64>().unwrap() * 10_f64.powf(ten_power2 as f64);
    let mut num_places2 = number2.len();
    if number2.contains('.') {
        num_places2 -= 1;
    }
    if number2.contains('-') {
        num_places2 -= 1;
    }
    
    //2.) perform the operation
    let mut result: f64 = 0.0;
    if operation == "*" {
        result = num1 * num2;
    } else if operation == "/" {
        result = num1 / num2;
    }
    
    if result < 0.0 {
        is_negative = true;
        result *= -1.0;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = cmp::min(num_places1, num_places2) as i32; //works
    
    
    let mut result_ten_power: i32 = 0;
    // println!("{}   ten power: {}", result, result_ten_power);
    let mut actual_dec_place: i32 = least_sigfigs.clone() - 1;
    if result < 10.0 {
        while result < 1.0 {
            result *= 10.0;
            result_ten_power -= 1;
            actual_dec_place += 1;
        }
    } else if result > 10.0 {
        while result > 10.0 {
            result /= 10.0;
            result_ten_power += 1;
            actual_dec_place -= 1;
        }
    }
    if is_negative {
        result *= -1.0;
    }
    let str_result = format!("{:.dec_places$}", result, dec_places=(least_sigfigs - 1) as usize);
    
    let mut error = ((error1.parse::<f64>().unwrap()/(num1)).powf(2.0) + 
                        (error2.parse::<f64>().unwrap()/(num2)).powf(2.0)).powf(0.5);
    // println!("{}      {}      {}", error, (error1.parse::<f64>().unwrap()/(num1)).powf(2.0), (error2.parse::<f64>().unwrap()/(num2)).powf(2.0));
    let raw_str_error = String::from(error.to_string());
    
    if error < 0.01 {
        let mut percent_dec_places = -2_i32;
        for (idx, letter) in raw_str_error.chars().enumerate() {   
            percent_dec_places += 1;
            if letter != '0' && letter != '.' {
                break;
            }
        }
        
        let percent = format!("{:.dec_places$}", error, dec_places=percent_dec_places as usize).parse::<f64>().unwrap();
        error = percent * str_result.parse::<f64>().unwrap() * 10_f64.powf(result_ten_power as f64);
        
    } else if error >= 0.01 {
        let percent = format!("{:.2}", error).parse::<f64>().unwrap();
        error = percent * str_result.parse::<f64>().unwrap() * 10_f64.powf(result_ten_power as f64);
    }
    
    let str_error = format!("{:.dec_places$}", error, dec_places=actual_dec_place as usize);
    // println!("error: {}   num1: {}    num2: {}", error, num1, num2);

    
    if !no_print {
        println!("{} E {}   +- {}", str_result, result_ten_power, str_error);
    }
    
    let result_vec: Vec<String> = vec![str_result, result_ten_power.to_string(), str_error];
    return result_vec


}

fn exp_error(number1: &str, ten_power1: i32, error1: &str, power: i32, no_print: bool) -> Vec<String>{
    //make sure that the errors properly match the measurement and the number of decimal places
    let valid_input1: bool = check_error(&number1, &error1, &ten_power1);
    
    if !valid_input1 {
        panic!("wtf are you doing with the msmts and errors");
    }

    // 1.) take care of the negative sign; take care of the number of sig figs for each number
    
    let num1: f64 = number1.parse::<f64>().unwrap() * 10_f64.powf(ten_power1 as f64);
    let mut num_places1 = number1.len();
    if number1.contains('.') {
        num_places1 -= 1;
    }
    if number1.contains('-') {
        num_places1 -= 1;
    }

    
    
    //2.) perform the operation
    let mut is_negative: bool = false;
    let mut result: f64 = num1.powf(power as f64);
    if result < 0.0 {
        is_negative = true;
        result *= -1.0;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = num_places1 as i32; //works
    
    
    let mut result_ten_power: i32 = 0;
    // println!("{}   ten power: {}", result, result_ten_power);

    let mut actual_dec_place: i32 = least_sigfigs.clone() - 1; //number of decimal places in sci notation

    if result < 10.0 {
        while result < 1.0 {
            result *= 10.0;
            result_ten_power -= 1;
            actual_dec_place += 1;
        }
    } else if result > 10.0 {
        while result > 10.0 {
            result /= 10.0;
            result_ten_power += 1;
            actual_dec_place -= 1;
        }
    }


    if is_negative {
        result *= -1.0;
    }
    let str_result = format!("{:.dec_places$}", result, dec_places=(least_sigfigs - 1) as usize);
    
    let mut error = error1.parse::<f64>().unwrap()/(num1) * power as f64;
    let raw_str_error = String::from(error.to_string());
    

    //error holds a percentage / 100%
    //this process rounds the percentage
    if error < 0.01 {
        let mut percent_dec_places = -2_i32;
        for (idx, letter) in raw_str_error.chars().enumerate() {   
            percent_dec_places += 1;
            if letter != '0' && letter != '.' {
                break;
            }
        }
        
        let percent = format!("{:.dec_places$}", error, dec_places=percent_dec_places as usize).parse::<f64>().unwrap();
        error = percent * str_result.parse::<f64>().unwrap() * 10_f64.powf(result_ten_power as f64);
        
    } else if error >= 0.01 {
        let percent = format!("{:.2}", error).parse::<f64>().unwrap();
        error = percent * str_result.parse::<f64>().unwrap() * 10_f64.powf(result_ten_power as f64);
    }
    
    let str_error = format!("{:.dec_places$}", error, dec_places=actual_dec_place as usize);
    // println!("error: {}   num1: {}    num2: {}", error, num1, num2);

    
    if !no_print {
        println!("{} E {}   +- {}", str_result, result_ten_power, str_error);
    }

    let result_vec: Vec<String> = vec![str_result, result_ten_power.to_string(), str_error];
    return result_vec
    
}



fn constant_mult_err(number1: &str, ten_power1: i32, error1: &str, constant: i32, no_print: bool) -> Vec<String>{
    //make sure that the errors properly match the measurement and the number of decimal places
    let valid_input1: bool = check_error(&number1, &error1, &ten_power1);
    
    if !valid_input1 {
        panic!("wtf are you doing with the msmts and errors");
    }

    // 1.) take care of the negative sign; take care of the number of sig figs for each number
    
    //Getting the raw number ********
    let num1: f64 = number1.parse::<f64>().unwrap() * 10_f64.powf(ten_power1 as f64);

    let mut num_places1 = number1.len();
    if number1.contains('.') {
        num_places1 -= 1;
    }
    if number1.contains('-') {
        num_places1 -= 1;
    }

    
    
    //2.) perform the operation
    let mut is_negative: bool = false;
    let mut result: f64 = num1 * constant as f64;
    if result < 0.0 {
        is_negative = true;
        result *= -1.0;
    }
    //to check when putting into scientific notation, whether or not you have to move the decimal place back
    let least_sigfigs = num_places1 as i32; //works
    
    
    let mut result_ten_power: i32 = 0;
    // println!("{}   ten power: {}", result, result_ten_power);

    let mut actual_dec_place: i32 = least_sigfigs.clone() - 1; //number of decimal places in sci notation

    if result < 10.0 {
        while result < 1.0 {
            result *= 10.0;
            result_ten_power -= 1;
            actual_dec_place += 1;
        }
    } else if result > 10.0 {
        while result > 10.0 {
            result /= 10.0;
            result_ten_power += 1;
            actual_dec_place -= 1;
        }
    }


    if is_negative {
        result *= -1.0;
    }
    let str_result = format!("{:.dec_places$}", result, dec_places=(least_sigfigs - 1) as usize);
    
    let mut error = error1.parse::<f64>().unwrap() * constant as f64;
    

    //error holds a percentage / 100%
    //this process rounds the percentage
    
    
    let str_error = format!("{:.dec_places$}", error, dec_places=actual_dec_place as usize);
    // println!("error: {}   num1: {}    num2: {}", error, num1, num2);

    
    if !no_print {
        println!("{} E {}   +- {}", str_result, result_ten_power, str_error);
    }

    let result_vec: Vec<String> = vec![str_result, result_ten_power.to_string(), str_error];
    return result_vec
    
    
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
