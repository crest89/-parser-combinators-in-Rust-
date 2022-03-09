// クロージャの型推論を補助するための関数
// cf. https://github.com/rust-lang/rust/issues/70263#issuecomment-623169045
fn generalize_lifetime<T, F>(f: F) -> F
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    f
}
// Psrser<T> を Fn(&str) -> Option<(T, &str)> の別名(のようなもの)として定義する。
pub trait Parser<T>: Fn(&str) -> Option<(T, &str)> {}
impl<T, F> Parser<T> for F where F: Fn(&str) -> Option<(T, &str)> {}

// S の先頭にある整数をパースし、整数値と残りの文字列を返す。
// パースに失敗した場合は None を返す。
pub fn digits(s: &str) -> Option<(i64, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    match s[..end].parse() {
        Ok(value) => Some((value, &s[end..])),
        Err(_) => None,
    }
}

// 先頭の一文字が c であるときに成功して　()　を返すようなパーサを返す。
pub fn character(c: char) -> impl Parser<()> {
    generalize_lifetime(move |s| {
        let mut chars = s.chars();
        if chars.next() == Some(c) {
            Some(((), chars.as_str()))
        } else {
            None
        }
    })
}
// パーサーを受け取って、先頭の空白を読み飛ばすようにしたパーサーを返す
pub fn lexeme<T>(parser: impl Parser<T>) -> impl Parser<T> {
    generalize_lifetime(move |s| parser(s.trim_start()))
}
// 特定の文字列をパースするパーサー
pub fn string(target: &'static str) -> impl Parser<()> {
    generalize_lifetime(move |s| s.strip_prefix(target).map(|rest| ((), rest)))
}

#[test]
fn test_string() {
    let parser = string("hello");
    assert_eq!(parser("hello world"), Some(((), " world")));
    assert_eq!(parser("hell world"), None);
}
// パースの結果に関数を適用する: map
pub fn map<A, B>(parser: impl Parser<A>, f: impl Fn(A) -> B) -> impl Parser<B> {
    generalize_lifetime(move |s| parser(s).map(|(value, rest)| (f(value), rest)))
}
