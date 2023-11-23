#[cfg(test)]
use crate::token::{TokenWrap, TokenKind};

#[test]
pub fn test_mut() {
    let mut t1 = TokenWrap::new(TokenKind::KEYWORD, &['a'], 1);

    pub fn change(tk: TokenWrap, reset: &mut TokenWrap) {
        println!("change: {:?}", tk.to_string());
        let t2 = TokenWrap::new(TokenKind::KEYWORD, &['b'], 1);
        reset.ptr = t2.ptr;
    }

    change(t1.clone(), &mut t1);
    println!("main: {:?}", t1.to_string());
}

#[test]
pub fn test_vec() {
    let mut v = vec![1,2,3,4,5];
    let v2 = v.clone();
    v.insert(0, 1000);
    println!("{:?}", v);
    println!("{:?}", v2);
}

#[test]
pub fn test_num() {
    let n = 1;
    for b in u32::to_be_bytes(n){
        println!("{:?}", b);
    }
}