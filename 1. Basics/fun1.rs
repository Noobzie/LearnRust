// fun1.rs

fn sqr(x: f64) -> f64 {
    return x * x;
}

fn main() {
    let res = sqr(2.0);
    println!("Square is {}", res);

    let res1 = abs(1.2);
    println!("Abs is {}", res1);

    let res2 = clamp(1.1, 2.2, 3.3);
    println!("Clamp is {}", res2);

    let res3 = factorial(3);
    println!("Factorial is {}", res3);

    let i = 10;
    let res4 = by_ref(&i);
    let res5 = by_ref(&41);
    println!("{} {}", res4, res5);
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2:f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial(n :u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}