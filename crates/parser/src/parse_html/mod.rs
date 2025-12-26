use html5ever::{
    local_name,
    tendril::StrTendril,
    tokenizer::{
        BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
    },
};
use std::cell::{Cell, RefCell};

use crate::console_log;

struct ParserState {
    in_p: Cell<bool>,
    in_heading: Cell<Option<u8>>,
    // in_person: Cell<bool>,
    // in_italic: Cell<bool>,
    // after_person: Cell<bool>,
    header_count: Cell<usize>,
    buffer: RefCell<String>,
    current_line: Cell<usize>,
}

pub struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState {
                in_p: Cell::new(false),
                in_heading: Cell::new(None),
                header_count: Cell::new(0),
                buffer: RefCell::new(String::new()),
                current_line: Cell::new(0),
            },
        }
    }

    fn new_line(&self) {
        self.state
            .current_line
            .set(self.state.current_line.get() + 1);
    }
}

impl TokenSink for Parser {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        let p_local = local_name!("p");
        let h1_local = local_name!("h1");
        let h2_local = local_name!("h2");
        let br_local = local_name!("br");
        let span_local = local_name!("span");

        match token {
            Token::TagToken(tag) => match tag.kind {
                TagKind::StartTag => match tag.name {
                    n if n == p_local => {
                        self.state.in_p.set(true);
                        self.new_line();
                    }
                    n if n == h1_local => {
                        self.state.in_heading.set(Some(1));
                        self.new_line();
                    }
                    n if n == h2_local => {
                        self.state.in_heading.set(Some(2));
                        self.new_line();
                    }
                    n if n == br_local => {
                        if self.state.in_p.get() {
                            self.new_line();
                        }
                    }
                    n if n == span_local => {
                        if self.state.in_p.get() {
                            self.state.buffer.borrow_mut().push('[');
                        }
                    }

                    _ => {}
                },

                TagKind::EndTag => match tag.name {
                    n if n == p_local => {
                        let text = self.state.buffer.borrow().trim().to_string();
                        if !text.is_empty() {
                            console_log!("{} | {}", self.state.current_line.get(), text);
                        }
                        self.state.buffer.borrow_mut().clear();
                        self.state.in_p.set(false);
                    }

                    n if n == h1_local || n == h2_local => {
                        if let Some(level) = self.state.in_heading.get() {
                            let text = self.state.buffer.borrow().trim().to_string();
                            if !text.is_empty() {
                                let count = self.state.header_count.get() + 1;
                                self.state.header_count.set(count);

                                console_log!(
                                    "{} | h{} | {}",
                                    self.state.current_line.get(),
                                    level,
                                    text
                                );
                            }
                        }
                        self.state.buffer.borrow_mut().clear();
                        self.state.in_heading.set(None);
                    }
                    n if n == span_local => {
                        if self.state.in_p.get() {
                            self.state.buffer.borrow_mut().push(']');
                        }
                    }
                    _ => {}
                },
            },

            Token::CharacterTokens(chars) => {
                if self.state.in_p.get() || self.state.in_heading.get().is_some() {
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
