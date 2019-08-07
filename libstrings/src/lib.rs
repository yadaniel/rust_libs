#![allow(unused)]

pub enum Anystr<'a> {
    StrA(&'a str),
    StrB(String)
}

pub fn concat(s1: Anystr, s2: Anystr) -> String {
    let s = String::new();
    match (s1,s2) {
        (Anystr::StrA(x), Anystr::StrA(y)) => concat_str(x,y),
        (Anystr::StrB(x), Anystr::StrB(y)) => concat_string(x,y),
        (Anystr::StrA(x), Anystr::StrB(y)) => concat_string(x.to_string(),y),
        (Anystr::StrB(x), Anystr::StrA(y)) => concat_string(x,y.to_string()),
    }
}

pub fn concat_string(s1: String, s2: String) -> String {
    format!("{}{}", s1, s2)
}

pub fn concat_str(s1: &str, s2: &str) -> String {
    let mut s = String::new();
    s.push_str(s1);
    s.push_str(s2);
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::concat_string("foo".to_string(), "bar".to_string()), "foobar".to_string());
        assert_eq!(super::concat_str("foo", "bar"), "foobar");
        use super::Anystr::*;
        assert!(super::concat(StrA("foo"), StrA("bar")) == "foobar".to_string());
        assert!(super::concat(StrA("foo"), StrB("bar".to_string())) == "foobar".to_string());
        assert!(super::concat(StrB("foo".to_string()), StrA("bar")) == "foobar".to_string());
        assert!(super::concat(StrB("foo".to_string()), StrB("bar".to_string())) == "foobar".to_string());
    }
}
