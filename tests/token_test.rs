use captcha::token::Token;
use captcha::phx::Point;
#[test]
fn aes_decode() {
    let t = Token::new();
    let hello = b"hello";
    let r = t.aes_encode(hello);
    let a = r.unwrap();
    assert_eq!("8hREp9FlQv6YlMWfLrx2TA==", a.clone());
    let rr: Vec<u8> = t.aes_decode(a.clone().as_str()).unwrap();
    assert_eq!(hello.to_vec(), rr)
}
#[test]
fn aes_decode_point(){
    let t = Token::new();
    let point = b"{\"x\":1, \"y\":2}";
    let r = t.aes_encode(point).unwrap();
    let rr: Point = t.aes_decode(r.as_str()).unwrap();
    assert_eq!(rr.x, 1);
    assert_eq!(rr.y, 2);
}
