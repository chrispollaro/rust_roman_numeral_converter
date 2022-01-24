fn to_roman_numeral(decimal: u32) -> String {
    let mut n = decimal;
    let mut s = String::new();
    let ords = vec![("IX","V","IV","I"),
                                              ("XC","L","XL","X"),
                                              ("CM","D","CD","C")];

    let mut r = n / 1000_u32;
    for _ in 0..r {
        s.push_str("M");
    }

    for (i, o)  in ords.iter().enumerate().rev() {
        let exp = i as u32;
        n = n - r * 10_u32.pow(exp + 1);
        r = n / 10_u32.pow(exp);
        
        match r {
            9 => s.push_str(o.0),
            x @ 5..=8 => { s.push_str(o.1); for _ in 0..(x - 5){s.push_str(o.3)}},
            4 => s.push_str(o.2),
            x @ 1..=3 => { for _ in 0..x{s.push_str(o.3)}},
            _ => (),
        }
    }

    s
}

fn main() {
    let n = 1978;
    println!("{} converted is {}", n, to_roman_numeral(n));
}

