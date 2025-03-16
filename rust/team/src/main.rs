use std::io;

fn compute(input: String) -> bool {
    let all = input.split_whitespace();
    all.map(|x| match x.parse::<i32>() {
        Ok(parsed) => parsed,
        Err(e) => panic!("{}", e),
    }).sum::<i32>() >= 2
}

#[test]
fn test_compute() {
    assert_eq!(compute("0 0 0".to_string()), false);
    assert_eq!(compute("0 0 1".to_string()), false);
    assert_eq!(compute("0 1 0".to_string()), false);
    assert_eq!(compute("0 1 1".to_string()), true);
    assert_eq!(compute("1 0 0".to_string()), false);
    assert_eq!(compute("1 0 1".to_string()), true);
    assert_eq!(compute("1 1 0".to_string()), true);
    assert_eq!(compute("1 1 1".to_string()), true);
}

fn readline(stdin: &io::Stdin) -> String {
    let mut buf = String::new();
    let _ = stdin.read_line(&mut buf);

    return buf.trim().to_string();
}

fn main() {
    let stdin = io::stdin();
    let linecount = readline(&stdin).trim().parse::<i32>().unwrap();
    let mut sure_amount = 0;

    for _ in 0..linecount {
        let line = readline(&stdin);

        if compute(line) {
            sure_amount += 1;
        }
    }

    println!("{}", sure_amount)
}
