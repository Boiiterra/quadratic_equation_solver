use std::io::Write;

fn input(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();

    let mut output: String = String::new();
    std::io::stdin()
        .read_line(&mut output)
        .expect("Invalid Unicode character");
    output
}

fn get_i32(msg: &str) -> i32 {
    loop {
        match input(msg).trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn quad_solver(a: i32, b: i32, c: i32) {
    let d: i32 = b * b - 4 * a * c;
    if d < 0 {
        println!("This equation has no roots.");
    } else if d == 0 {
        let ans: f64 = -b as f64 / (2f64 * a as f64);
        println!("Found the only root of the equation -> {}", ans);
    } else {
        let mut ans: f64 = (-b as f64 - (d as f64).sqrt()) / (2f64 * a as f64);
        println!("First root  -> {}", ans);
        ans = (-b as f64 + (d as f64).sqrt()) / (2f64 * a as f64);
        println!("Second root -> {}", ans)
    }
}

fn part_quad_solver(a: i32, c: i32) {
    if -c < 0 {
        println!("No roots found.");
    } else {
        let ans: f64 = (-c as f64 / a as f64).sqrt();
        println!("First root  -> {}", ans);
        println!("Second root -> {}", -ans);
    }
}

fn linear_solver(b: i32, c: i32) {
    if c == 0 {
        println!("Root -> 0");
    } else {
        let ans: f64 = -c as f64 / b as f64;
        println!("Root -> {}", ans);
    }
}

fn main() {
    println!("Using normal quadratic equation (0): ax² + bx + c = 0");

    let a: i32 = get_i32("a = ");
    let b: i32 = get_i32("b = ");
    let c: i32 = get_i32("c = ");

    if a != 0 && b == 0 && c != 0 {
        part_quad_solver(a, c);
    } else if a != 0 {
        quad_solver(a, b, c);
    } else if a == 0 && b != 0 {
        linear_solver(b, c);
    } else {
        println!("This is weird... -> {} = 0", c);
    }
}
