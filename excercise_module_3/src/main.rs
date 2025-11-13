fn main() {
    // temperature_coverter();
    fibonacci();
}

fn temperature_coverter() {
    println!("Temperature converter!");

    let mut input: String = String::new();

    println!("Enter temperature caliberation to convert to");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to pass input");

    let caliberation: char = input
        .trim()
        .parse::<char>()
        .expect("Unable to parse input");


    println!("Enter the numeric temperature");

    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to pass input");

    let temperature: f32 = input
        .trim()
        .parse::<f32>()
        .expect("Unable to parse input");
    
    // println!("The temperature is : {}{}", temperature_coverter_init(temperature, &caliberation), caliberation);
}

// Temperaature converter
fn temperature_coverter_init(temp: f32, to: &char) -> f32 {
    if *to == 'C' {
        return convert_to_celcius(temp);
    } else if *to == 'F' {
        return convert_to_farenheit(temp);
    } else {
        println!("Measure not supported");
        return 0.0;
    }
}

fn convert_to_celcius(temp: f32) -> f32 {
    ((temp - 32.0) * 5.0) / 9.0
}

fn convert_to_farenheit(temp: f32) -> f32 {
    ((temp * 9.0) / 5.0) + 32.0
}


// Fibonnaci printer
fn fibonacci() {
    println!("Fibonnaci printer");

    println!("Enter the no of fibonnacis to print");

    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to pass input");

    let fib_n: u32 = input
        .trim()
        .parse::<u32>()
        .expect("Unable to parse input");
    print_fibonacci_init(fib_n);
}

// Fibonacci numbers
fn print_fibonacci_init(n: u32) {
    let mut first = 1;
    let mut second = 1;
    let mut num;

    if n == 1 {
        println!("Fibonacci {}", first);
    } else if n == 2 {
        println!("Fibonacci {}", first);
        println!("Fibonacci {}", second);
    } else {
        println!("Fibonacci {}", first);
        println!("Fibonacci {}", second);

        let mut x = 2;
        while x < n {
            num = first + second;
            first = second;
            second = num;

            println!("Fibonacci {}", num);
            x+=1;
        }
    }
}

