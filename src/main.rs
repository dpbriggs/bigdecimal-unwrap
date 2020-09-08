use bigdecimal::BigDecimal;

fn main() {
    let n = BigDecimal::from(1);
    let d = BigDecimal::from(3);
    println!("{}", (n / d).round(0));
}
