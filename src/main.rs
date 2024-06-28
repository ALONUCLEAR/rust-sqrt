// This code will calculate a square root based on Heron's method

fn main() {
    let sqrt_pi = f32::sqrt(std::f32::consts::PI);
    let sqrt_max = f32::sqrt(f32::MAX);

    let test_sqrts = vec![1.0, 2.0, 3.14, sqrt_pi, 53.0, 0.7, 10000.0, sqrt_max];
    let fails = test(test_sqrts);

    println!("Custom sqrt function failed {fails} times");
}

fn test(sqrts: Vec<f32>) -> i32 {
    let mut fails = 0;

    for val in sqrts {
        let square = val * val;
        let guess = heron(square, 3.0, 1000);

        if guess == val {
            println!("Got it right! sqrt({square}) is indeed {guess}");
        } else {
            fails += 1;
            println!("You were a bit off... sqrt({square}) is {val}, not {guess}");
        }
    }

    return fails;
}

// Heron's Method For a number S, guess it's sqrt is K
// Mark every guess as Xn, and X0= K
// X(n+1) = (Xn + S/Xn)/2
fn heron(num: f32, initial_guess: f32, max_iterations: u32) -> f32 {
    const SURE_GUESS_COUNT: u32 = 3;

    let mut last_guess = initial_guess;
    let mut matching_guesses = 0;
    let mut its = 0;

    while matching_guesses < SURE_GUESS_COUNT && its < max_iterations {
        let new_guess = (last_guess + num / last_guess) / 2.0;

        if last_guess == new_guess {
            matching_guesses += 1;
        } else {
            matching_guesses = 0;
        }

        last_guess = new_guess;
        its += 1;
    }

    its -= SURE_GUESS_COUNT;

    println!("Took {its} iterations to get the sqrt of {num}");

    return last_guess;
}
