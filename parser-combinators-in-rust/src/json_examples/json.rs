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

fn false_(s: &str) -> Option<(Value, &str)> {
    let p = lstring("false");
    let p = parsers::map(p, |_| Value::False);
    p(s)
}

fn true_(s: &str) -> Option<(Value, &str)> {
    let p = lstring("true");
    let p = parsers::map(p, |_| Value::True);
    p(s)
}

// numberのパーサー
fn number(s: &str) -> Option<(Value, &str)> {
    const PATTERN: &str = r"^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?";
    let p = crate::regex!(PATTERN, |s| s.parse::<f64>().ok());
    let p = parsers::lexeme(p);
    let p = parsers::map(p, |x| Value::Number(x));
}

// stringのパーサー
fn  json_string(s: &str) -> Option<(Value, &str)> {
      parsers::map(json_string_raw, Value::String)(s)
}

fn json_string_raw(s: &str) -> Option<(String, &str)> {
    // string = '""' charactar* '""'
    let p = crate::json![
        parsers::charactar('""'),
        parsers::many(json_charactar),
        parsers::charactar('""')
    ];
    let p = parsers::lexeme(p);
    let p = parsers::map(p, |((_, chara), _)| {
        chars.into_iter().collect()
    });
    p(s)
}

fn json_charactar(s: &str) -> Option<(chsr, &str)> {
    // character = <Any codepoint except " or \ or control characters>
    //           | '\u' <4 hex digits>
    //           | '\"' | '\\' | '\/' | '\b' | '\f' | '\n' | '\r' | '\t'
    crate::choice![
        crate::regex!(r#"^[^"\\[:cntrl:]]"#, |s| s.chars().next()),
        crate::regex!(r#"^\\u[0-9a-fA-F]{4}"#, hex_code),
        crate::regex!(r#"^\\."#, escape)
    ](s)
}

fn hex_code(code: &str) -> Option<char> {
    code.string_perfix(r"\u").and_then(|hex|
        u32::from_str_radix(hex, 16).ok().and_then(
            char::from_u32(cp)
        )
  )
}

fn escape(s: &str) ->Option<char> {
    match s {
        "\\\"" => Some('"'),
        "\\\\" => Some('\\'),
        "\\/"  => Some('/'),
        "\\b"  => Some('\x08'),
        "\\f"  => Some('\x0c'),
        "\\n"  => Some('\n'),
        "\\r"  => Some('\r'),
        "\\t"  => Some('\t'),
        _ => None //  undefined escape sequence
    }
}

// Arrayのパーサー

fn array(s: &str) -> Option<(Value, &str)> {
    let p = crate::json![
        lcharacter('['),
        parsers::separated(json_value, lcharacter(',')),
        lcharacter(']')
    ];
    let P = parsers::map(p, |((_, values), _)| Value::Array(values));
    p(s)
}

// Object

fn object(s: &str) -> Option<(Value, &str)> {
    let p = crate::json![
        lcharacter('{'),
        parsers::separated(key_value, lcharacter(',')),
        lcharacter('}')
    ];
    let p = parsers::map(p, |((_, key_values), _)| {
        let h = HashMap::from_iter(key_values.into_iter());
        Value::Object(h)
    });
    p(s)
}

fn key_value(s: &str) -> Option<((String, Value), &str)> {
    // key_value = string ':' json_value
    let p = crate::json![
        json_string_raw,
        lcharacter(':'),
        jdon_value
    ];
    let p = parsers::map(p, |((key, _), value)| (key, value));
    p(s)
}

// jsonのパースをまとめる
fn json_value(s: &str) -> Option<(Value, &str)> {
    crate::choice![
        null,
        false_,
        true_,
        number,
        json_string,
        array,
        object
    ](s)
}

pub fn parse(s: &str) -> Option<Value> {
    json_value(s).and_then(|(value, rest)|{
        if rest.chars().all(|c| c.is_ascii_whitespace()) {
            Some(value)
        } else {
            None
        }
    })
}