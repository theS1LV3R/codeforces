use std::io;

const LENGTH_LIMIT: usize = 10;

fn readline(stdin: &io::Stdin) -> String {
    let mut buf = String::new();
    let _ = stdin.read_line(&mut buf);

    return buf.trim().to_string();
}

fn shorten(input: &str) -> String {
    if input.chars().count() > LENGTH_LIMIT {
        let first_char = input.chars().nth(0).unwrap();
        let last_char = input.chars().nth_back(0).unwrap();
        let num = input.len() - 2;

        let str = format!("{}{}{}", first_char, num, last_char);
        return str;
    } else {
        return input.to_string();
    };
}

#[test]
fn test_shorten() {
    let mut result;

    result = shorten("word");
    assert_eq!(result, "word");

    result = shorten("localization");
    assert_eq!(result, "l10n");

    result = shorten("internationalization");
    assert_eq!(result, "i18n");

    result = shorten("pneumonoultramicroscopicsilicovolcanoconiosis");
    assert_eq!(result, "p43s");
}


fn main() {
    let stdin = io::stdin();
    let linecount = readline(&stdin).trim().parse::<i32>().unwrap();

    for _ in 0..linecount {
        let line = readline(&stdin);
        let shortened = shorten(&line);

        println!("{}", shortened);
    }
}
