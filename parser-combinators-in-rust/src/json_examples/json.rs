#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    False,
    True,
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

// 便利関数の定義
// parsersモジュールの呼び出し
fn lstring(target: &'static str) -> impl parsers::Parser<()> {
    parsers::lexeme(parsers::string(target))
}

fn llcharacter(c: char) -> impl parsers::Parser<()> {
    parsers::lexeme(parsers::lcharacter(c))
}

// null, false, trueのパーサー
fn null(s: &str) -> Option<(Value, &str)> {
    let p = lstring("null");
    let p = parsers::map(p, |_| Value::Null);
    p(s)
}

fn false(s: &str) -> Option<(Value, &str)> {
    let p = lstring("false");

}