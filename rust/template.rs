use std::io;

fn compute(input: String) -> bool {
  false
}

#[test]
fn test_compute() {
    let mut result = compute("".to_string());
    assert_eq!(result, false);
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

        ....
    }

    println!("{}", sure_amount)
}
