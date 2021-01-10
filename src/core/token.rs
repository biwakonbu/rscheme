#[derive(Debug, Clone)]
pub enum Token {
    LeftParen,
    RightParen,

    // 四則演算記号 +-*/
    Add, Sub, Mul, Div,

    // Type
    Boolean(String),
    Number(String),
    // Character(String),
    // String(String),
    // Vector(String),

    // keywords
    Quote,
    Lambda,
    If,
    SetEx,
    Begin,
    Cond,
    And,
    Or,
    Case,
    Let,
    LetStar,
    LetRec,
    Do,
    Delay,
    Quasiquote
}

#[derive(Debug, Clone)]
pub struct AST {
    tokens: Vec<Token>
}

impl AST {
    pub fn new() -> Self {
        return AST { tokens: Vec::new() };
    }

    fn add(&mut self, buffer: String) {
        match self.str_to_token(buffer) {
            Ok(token) => {
                self.add_token(token);
            },
            Err(s) => {
                println!("{}", s);
            }
        }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn is_number(&self, buffer: String) -> Result<bool, String> {
        let mut float_number = false;

        for c in buffer.chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {},
                '.' => {
                    if float_number {
                        return Err(format!("TokenError: Wrong Token::Number format {}", buffer));
                    } else {
                        float_number = true
                    }
                },
                _ => return Ok(false),
            }
        }
        return Ok(true);
    }

    fn str_to_token(&self, buffer: String) -> Result<Token, String> {
        match self.is_number(buffer.clone()) {
            Ok(result) => {
                if result {
                    return Ok(Token::Number(buffer));
                }
            },
            Err(s) => return Err(s),
        }

        match &*buffer {
            // 四則演算
            "+"          => return Ok(Token::Add),
            "-"          => return Ok(Token::Sub),
            "*"          => return Ok(Token::Mul),
            "/"          => return Ok(Token::Div),
            // Boolean
            "#t" | "#n"  => return Ok(Token::Boolean(buffer)),
            // keywords
            "quote"      => return Ok(Token::Quote),
            "lambda"     => return Ok(Token::Lambda),
            "if"         => return Ok(Token::If),
            "and"        => return Ok(Token::And),
            "or"         => return Ok(Token::Or),
            "case"       => return Ok(Token::Case),
            "set!"       => return Ok(Token::SetEx),
            "begin"      => return Ok(Token::Begin),
            "cond"       => return Ok(Token::Cond),
            "let"        => return Ok(Token::Let),
            "let*"       => return Ok(Token::LetStar),
            "letrec"     => return Ok(Token::LetRec),
            "do"         => return Ok(Token::Do),
            "delay"      => return Ok(Token::Delay),
            "quasiquote" => return Ok(Token::Quasiquote),
            _            => return Err(buffer),
        }
    }
}

pub fn tokenize(line: &str) -> AST {
    let mut ast = AST::new();
    let mut buffer: String = String::new();
    let mut comment_mode = false;

    for c in line.chars() {
        if comment_mode && c != '\n' {
            // コメントモード自は改行コードが来るまで全て無視
            continue;
        }

        match c {
            '(' => {
                ast.add_token(Token::LeftParen);
            },
            ')' => {
                ast.add(buffer.clone());
                buffer.clear();

                ast.add_token(Token::RightParen);
            },
            // 空白文字が来たらトークンを確定させる
            ' ' => {
                ast.add(buffer.clone());
                buffer.clear();
            },
            // ; の後ろは改行文字まで全て無視
            ';' => {
                comment_mode = true;
            },
            // 改行 '\n' が来たらコメント行終了
            '\n' => {
                comment_mode = false;
            },
            _ => {
                buffer.push(c);
            },
        }
    }

    println!("{:?}", ast);

    return ast;
}
