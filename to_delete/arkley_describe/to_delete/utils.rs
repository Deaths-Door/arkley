pub(super) const SEPERATOR : &str = r"& \hline\\";
pub(super) const BASE : u32 = 10;
pub(super) const SPACE : char = ' ';

/* const DESCRIPTION : &str =  "Multiply each digit in the second number with the digits in the first number, and write the results below each digit in the second number.";

        let mut solution = SolutionSteps::new();

        let mut step1 = Step::new(DESCRIPTION.to_string());
       
        let x_neg = self.is_negative();
        let y_neg = other.is_negative();

        let mut either_neg : bool = false;
        
        if x_neg && x_neg {
            step1.insert_to_description("\nSince both are negative we can simpilfy them into positive");
        }
        else if x_neg || y_neg {
            either_neg = true;
            step1.insert_to_description("\nSince we have a negative number lets ignore the sign for now.");
        }

        let (x,y) = swap_if_greater(self.abs(),other.abs());

        let (padding,longest_decimal)  = figure_alignment(x,y);

        let c_aligned = into_column(x,y,"+",padding,longest_decimal);

        let c_aligned_joined = c_aligned.join("");

        let space_index = c_aligned[0].find(' ').unwrap();

        let _c_zero = &c_aligned[0];
        let _c_one = &c_aligned[1];

        // So only valid numbers are there in loop
        let x_str = &_c_zero[space_index.._c_zero.len() - 3].trim_start_matches(SPACE);
        let y_str = &_c_one[space_index + 2.._c_zero.len() - 3].trim_start_matches(SPACE);

        // to take into account decimal points in numbers for the factor scaling so 10 to the power of index - (encounter as i32)
        let mut x_dec_encounted = false;
        let mut y_dec_encounted = false;

        // for sums of mul
        let mut sum = String::new();

        for  (y_index,y_ch) in y_str.chars().rev().enumerate() {
            if y_ch == '.' {
                y_dec_encounted = true;
                continue;
            }

            let yd = y_ch.to_digit(BASE).unwrap();

            if yd == 0 {
                if y_index == y_str.len() - 1 {
                    continue;    
                }
                
                let format = format!("Now we can skip multiplying {x_str} with 0 as {x_str} * 0 = 0");
                let substep = SubStep::new(format);
                step1.add_substep(substep);
                continue;
            }

            let y_factor = 10_u32.pow(y_index as u32 - y_dec_encounted as u32);

            let y_digit = yd * y_factor;

            for (x_index,x_ch) in x_str.chars().rev().enumerate() {
                if x_ch == '.' {
                    x_dec_encounted = true;
                    continue;
                }

                let xd = x_ch.to_digit(BASE).unwrap();

                if xd == 0 {
                    let format = format!("Now we can skip multiplying 0 with {y_str} as 0 * {y_str} = 0");
                    let substep = SubStep::new(format);
                    step1.add_substep(substep);
                    continue;
                }

                let x_factor = 10_u32.pow(x_index as u32 - x_dec_encounted as u32);
                
                let x_digit = xd * x_factor;

                let product = y_digit * x_digit;

                sum += &align(product as f64,padding,longest_decimal);

                let description = format!(r"Multiply ${y_digit} \times ${x_digit} which is {product}\nNow write the product down below");

                let latex = align_latex_end(&format!("{c_aligned_joined}{sum}"));

                let mut substep = SubStep::new(description);
                substep.set_latex(latex);

                step1.add_substep(substep);
            }
        }

        solution.push(step1);

        const START_ADD : &str = "Now lets add theh results of the mul together";        

        let mut step2 = Step::new(START_ADD.to_string());
        describe_add(&mut step2,sum);

        solution.push(step2);

        if either_neg {
            const INCLUDE_NEG : &str = "We previously omitted the negative sign, but now we've included it into the sum. ";
            let step3 = Step::new(INCLUDE_NEG.to_string());
            solution.push(step3);
        }
        
        Some(solution)*/
    
pub(super) fn swap_if_greater(x: f64, y: f64) -> (f64, f64) {
    match x > y {
        true => (x, y),
        false => (y, x)
    } 
}
pub(super) fn figure_alignment(x : f64,y : f64) -> (usize,usize) {
    let x_str = x.to_string();
    let y_str = y.to_string();

    let padding = x_str.len().max(y_str.len()) + 6;

    let mut longest_decimal : usize = 0 ;

    let mut closure = |string : &str|{
        let index = string.find('.').unwrap();
        let dec_len = string.len() - index - 1;

        if dec_len > longest_decimal {
            longest_decimal = dec_len
        };
    };
    
    if x.fract() != 0.0 {
        closure(&x_str);
    }

    if y.fract() != 0.0 {
        closure(&y_str);
    }

    (padding,longest_decimal)
}

pub(super) fn align(number : f64,padding : usize,longest_decimal : usize) -> String {
    format!(r"& {:width$.dec$} \\",number,width = padding,dec = longest_decimal)
}

pub(super) fn into_column(x : f64,y : f64,op_str : &str,padding : usize,longest_decimal : usize) -> Vec<String> {
    let padded_x = align(x,padding,longest_decimal);
    let padded_y = format!(r"{} & {:width$.dec$} \\",op_str,y,width = padding - 2,dec = longest_decimal);


    return vec![padded_x,padded_y,SEPERATOR.to_string()]
}

pub(super) fn align_latex_end(latex : &str) -> String {
    format!(r"\begin{{alignat*}}{{1}} {latex} \end{{alignat*}}")
}