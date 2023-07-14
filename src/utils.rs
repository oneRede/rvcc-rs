#[allow(dead_code)]
pub fn get_str_num(s: &str) -> (isize, &str){
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '0'..='9' => {i += 1;},
            '-' | '+' => {
                break;
            }
            _ => {}
        }
    }

    return ((&s[..i]).parse().unwrap(), &s[i..])
}

#[test]
fn test_get_str_num() {
    let s = "12335+67890";
    let (a,b) = get_str_num(s);
    println!("{}", a);
    println!("{}", b);
}
