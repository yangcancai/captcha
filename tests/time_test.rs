use captcha::{self, time};
#[test]
fn epoch(){
    let s = time::epoch();
    let str = s.to_string();
    assert_eq!(str.len(), 13);
}