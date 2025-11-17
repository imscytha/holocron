#[allow(dead_code)] // FIXME: temporarily ignore warning
#[derive(Debug, PartialEq)]
enum Token {
    OpenTag {
        name: String,
        attributes: Vec<(String, String)>,
    },
    EmptyTag {
        name: String,
        attributes: Vec<(String, String)>,
    },
    CloseTag(String),
    Text(String),
    Eof,
}

struct Tokenizer<'a> {
    rest: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { rest: input }
    }

    fn peek(&mut self) -> Option<char> {
        self.rest.chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        let ch = chars.next()?;

        let len = ch.len_utf8();
        self.rest = &self.rest[len..];

        Some(ch)
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &'a str {
        let len = self
            .rest
            .chars()
            .take_while(|&ch| predicate(ch))
            .map(|ch| ch.len_utf8())
            .sum();

        let (consumed, rest) = self.rest.split_at(len);
        self.rest = rest;

        consumed
    }

    fn read_tag_name(&mut self) -> String {
        let name_slice = self.consume_while(|ch| !ch.is_whitespace() && ch != '>' && ch != '/');
        name_slice.to_string()
    }

    fn next_token(&mut self) -> Token {
        self.consume_while(|ch| ch.is_whitespace());

        match self.peek() {
            Some('<') => {
                self.next_char();

                if let Some('/') = self.peek() {
                    self.next_char();
                    let name = self.read_tag_name();
                    self.consume_while(|ch| ch != '>');
                    self.next_char();
                    return Token::CloseTag(name);
                }

                let name = self.read_tag_name();
                let attributes = vec![]; // placeholder for now
                self.consume_while(|ch| ch.is_whitespace());

                if let Some('/') = self.peek() {
                    self.next_char();
                    self.consume_while(|ch| ch.is_whitespace());
                    self.next_char();
                    return Token::EmptyTag { name, attributes };
                }

                if let Some('>') = self.peek() {
                    self.next_char();
                    return Token::OpenTag { name, attributes };
                }

                panic!("Malformed tag structure!");
            }
            Some(tbd) => {
                todo!("{tbd} is not implemented yet!");
            }
            None => Token::Eof,
        }
    }
}

fn main() {
    let s = String::from("<node></node>");
    let mut tokenizer = Tokenizer::new(&s);
    loop {
        let token = tokenizer.next_token();
        println!("{token:?}");

        if token == Token::Eof {
            break;
        }
    }
}
