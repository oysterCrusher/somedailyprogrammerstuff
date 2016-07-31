pub fn simplify(input: &'static str) -> (i32, i32) {
    let inputs: Vec<&str> = input.split(" ").collect();
    println!("{:?}", inputs);
    let j: Vec<i32> = inputs.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    simplify_fraction(j[0], j[1])
}

fn simplify_fraction(numerator: i32, denominator: i32) -> (i32, i32) {
    let gcd = gcd(numerator, denominator);
    (numerator / gcd, denominator / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use simplify;

    #[test]
    fn simplify_test() {
        assert_eq!((1, 2), simplify("4 8"));
        assert_eq!((64, 3265), simplify("1536 78360"));
        assert_eq!((25739, 2768), simplify("51478 5536"));
        assert_eq!((7, 18), simplify("46410 119340"));
        assert_eq!((7673, 4729), simplify("7673 4729"));
        assert_eq!((4, 1), simplify("4096 1024"));
    }
}
