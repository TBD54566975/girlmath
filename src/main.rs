use girlmath;

fn main() {
    let minor_units = girlmath::major_to_minor("USD", "30").unwrap();
    println!("{}", minor_units);
}
