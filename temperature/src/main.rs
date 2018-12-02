#[derive(Debug)]
struct Celsius(f32);
#[derive(Debug)]
struct Fahrenheit(f32);

fn main() {
    println!("{:?}", Celsius::from(Fahrenheit(0.0)));
    println!("{:?}", Celsius::from(Fahrenheit(32.0)));
    println!("{:?}", Celsius::from(Fahrenheit(212.0)));

    println!("{:?}", Fahrenheit::from(Celsius(0.0)));
    println!("{:?}", Fahrenheit::from(Celsius(100.0)));

    println!("{:?}", Celsius::from(Fahrenheit(-40.0)));
    println!("{:?}", Fahrenheit::from(Celsius(-40.0)));
}

impl From<Celsius> for Fahrenheit {
    fn from(temp: Celsius) -> Fahrenheit {
        Fahrenheit((temp.0 * 9.0 / 5.0) + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(temp: Fahrenheit) -> Celsius {
        Celsius((temp.0 - 32.0) * 5.0 / 9.0)
    }
}
