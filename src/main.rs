use std::ops::Not;

#[allow(unused)]
#[derive(Clone, Debug)]
enum Token {
    Tadi,
    OpenParen,
    CloseParen,
    Eq,
    Dot,
    NumLit(f64),
    Ident(String),
}

#[derive(Clone, Debug)]
struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}
impl Scanner {
    pub fn new(text: String) -> Self {
        Self {
            chars: text.chars().collect(),
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn run(mut self) -> Vec<Token> {
        while let Some(char) = self.advance() {
            match (char, char.is_alphabetic(), char.is_numeric()) {
                // single char tokens
                ('(', _, _) => self.add_token(Token::OpenParen),
                (')', _, _) => self.add_token(Token::CloseParen),
                ('=', _, _) => self.add_token(Token::Eq),
                ('.', _, _) => self.add_token(Token::Dot),

                // ignor chars
                (' ' | '\n' | '\r', _, _) => {}

                // numbers
                (_, false, true) => {
                    while let Some(char) = self.peek_char()
                        && (char.is_numeric() || char == '.')
                    {
                        self.advance().unwrap();
                    }
                    dbg!(&self.tokens);
                    self.add_token(Token::NumLit(
                        dbg!(self.peek_lexeme().unwrap()).parse().unwrap(),
                    ));
                }

                (_, true, false) => {
                    while let Some(char) = self.peek_char()
                        && (char.is_alphanumeric() || char == '_')
                    {
                        self.advance().unwrap();
                    }
                    let word = self.peek_lexeme().unwrap();
                    match word.as_str() {
                        "tadi" => self.add_token(Token::Tadi),
                        _ => self.add_token(Token::Ident(word)),
                    }
                }

                _ => todo!(),
            }
            self.start = self.current;
        }
        self.tokens
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        self.done().not().then(|| {
            self.current += 1;
            self.chars[self.current - 1]
        })
    }

    fn peek_char(&self) -> Option<char> {
        self.done().not().then(|| self.chars[self.current])
    }

    fn peek_lexeme(&self) -> Option<String> {
        self.done()
            .not()
            .then(|| String::from_iter(&self.chars[self.start..self.current]))
    }

    fn done(&self) -> bool {
        self.current == self.chars.len()
    }
}

fn main() {
    let text = "tadi first.test (avg)var1=1\nother log\n tadi first.test (avg)var1=2\n";
    let scanner = Scanner::new(text.to_string());
    let tokens = scanner.run();
    dbg!(tokens);
}
