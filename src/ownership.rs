pub fn run() {

    let s = String::from("Hello world");
    let res = first_word(&s);

    println!("{}", res);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s;
}