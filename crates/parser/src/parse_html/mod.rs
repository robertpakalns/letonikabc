use html5ever::{
    local_name,
    tendril::StrTendril,
    tokenizer::{
        BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
    },
};
use std::cell::RefCell;

use crate::console_log;

struct ParserState {
    stack: RefCell<Vec<String>>,
    buffer: RefCell<String>,
}

pub struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState {
                stack: RefCell::new(Vec::new()),
                buffer: RefCell::new(String::new()),
            },
        }
    }
}

impl TokenSink for Parser {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        let span_local = local_name!("span");
        let p_local = local_name!("p");

        match token {
            Token::TagToken(tag) => match tag.kind {
                TagKind::StartTag => {
                    self.state.stack.borrow_mut().push(tag.name.to_string());

                    if tag.name == span_local {
                        self.state.buffer.borrow_mut().push('[');
                    }
                }
                TagKind::EndTag => {
                    if tag.name == span_local {
                        self.state.buffer.borrow_mut().push(']');
                    }

                    if tag.name == p_local {
                        let text = self.state.buffer.borrow().trim().to_string();
                        if !text.is_empty() {
                            console_log!("{}", text);
                        }
                        self.state.buffer.borrow_mut().clear();
                    }

                    self.state.stack.borrow_mut().pop();
                }
            },
            Token::CharacterTokens(chars) => {
                if self.state.stack.borrow().contains(&"p".to_string()) {
                    self.state.buffer.borrow_mut().push_str(&chars);
                }
            }
            _ => {}
        }

        TokenSinkResult::Continue
    }
}

pub fn parse(html: &str) {
    let mut input = BufferQueue::default();
    input.push_back(StrTendril::from(html));

    let tokenizer = Tokenizer::new(Parser::new(), TokenizerOpts::default());
    let _ = tokenizer.feed(&mut input);
    tokenizer.end();
}
