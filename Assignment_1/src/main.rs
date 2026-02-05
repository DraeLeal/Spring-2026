// Assignment 1: Temperature Converter
const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0/5.0) + FREEZING_POINT  
}

fn assignment_1(){

    let mut temp_f: f64 = FREEZING_POINT;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F = {:.2}°C", temp_f, temp_c);

   for _ in 0..5{
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F = {:.2}°C", temp_f, temp_c);
}
}

// Assignment_2
fn assignment_2(numbers: &[i32]){
    //Even or Odd function
    fn is_even(n: i32) -> bool{
    n % 2 == 0
}
    //loop to iterate array
    for num in numbers.iter(){
        // FizzBuzz
        if num % 3 == 0 && num % 5 == 0{
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0{
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0{
            println!("{}: Buzz", num);
        }
        // Even Or Odd
        else{
           if is_even(*num){
                println!("{}: Even", num);
            }
            else{
                println!("{}: Odd", num);
            }
        }

    }
    // Variables for Sum loop
    let mut index = 0;
    let mut sum = 0;
    // While loop for Sum
    while index < numbers.len(){
        sum += numbers[index];
        index += 1;
    }
    println!("Sum: {}", sum);

     //Variables for largest number loop
    let mut largest = numbers[0];
    let mut i = 1;
    // Loop to find largest number
    loop{
        if i >= numbers.len(){
            break;
        }
        if numbers[i] > largest{
            largest = numbers[i];
        }
        i+=1; 
    }
    println!("Largest number: {}", largest);

}

// Assignment 3 Guessing Game
fn assignment_3(){
    fn check_guess(guess: i32, secret: i32)->i32{
        if guess == secret{
            0
        }
        else if guess > secret{
            1
        }
        else{
            -1
        }
    }
    let secret: i32 = 7; 
    let mut guess: i32 = 0;
    let mut count: i32 = 0;

    loop{
        guess += 1;
        count += 1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("Guess {} is correct!", guess);
            break;
        }
        else if result == 1{
            println!("Guess {} is too high", guess);
        }
        else{
            println!("Guess {} is too low", guess);
        }   
    }
    println!("It took {} guesses.", count);
    }


fn main() {

    assignment_1();

    // Assignment 2 array
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assignment_2(&numbers);

    assignment_3();
   
}
