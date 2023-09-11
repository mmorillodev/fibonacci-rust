use std::io;

fn main() {
    let n: u32;

    loop {
        let mut input = String::new();

        println!("Enter an unassigned integer value:");

        io::stdin().read_line(&mut input).expect("Failed to read input.");

        n = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => continue
        };

        break;
    }

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    let mut curr = 1;
    let mut prev = 1;

    if n == 1 || n == 2 { return 1; }

    for _ in 0..n-2 {
        let aux = curr;
        curr = curr + prev;
        prev = aux;
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_the_fibonacci_of_a_given_number() {
        assert_eq!(fibonacci(6), 8);
    }

    #[test]
    fn should_return_the_fibonacci_of_2() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn should_return_the_fibonacci_of_1() {
        assert_eq!(fibonacci(1), 1);
    }
}