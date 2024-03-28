fn main() {
    let mut x: f32 = 3.00000011;
    //               0.1234567
// x =              x + 0.000000099999999999;
    let y = 3.123456789;

    println!("{x}");
}

// Review IEEE754 on why it uses the banker's rounding of half to even