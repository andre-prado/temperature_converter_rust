use std::io;

fn main() {
    let mut celsius = String::new();

    println!("Convert Celsius to Fahrenheit");
    println!("Please, enter the value for Celsius");


    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
    
    let celsius: f32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("You must enter a number!"),
    };
    
    
    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{}°C = {}°F", celsius, fahrenheit);

}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}
