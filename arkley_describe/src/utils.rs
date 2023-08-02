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

    const SEPERATOR : &str = r"& \hline\\";

    return vec![padded_x,padded_y,SEPERATOR.to_string()]
}

pub(super) fn align_latex_end(latex : &str) -> String {
    format!(r"\begin{{alignat*}}{{1}} {latex} \end{{alignat*}}")
}