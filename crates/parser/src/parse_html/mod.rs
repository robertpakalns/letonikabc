use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{
    BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

use crate::console_log;

struct Printer;

impl TokenSink for Printer {
    type Handle = ();

    // For now, output results into console
    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            Token::TagToken(tag) => match tag.kind {
                TagKind::StartTag => {
                    console_log!("StartTag: <{}>", tag.name);
                }
                TagKind::EndTag => {
                    console_log!("EndTag: </{}>", tag.name);
                }
            },
            Token::CharacterTokens(chars) => {
                let text = chars.trim();
                if !text.is_empty() {
                    console_log!("Text: {}", text);
                }
            }
            Token::CommentToken(comment) => {
                console_log!("Comment: <!--{}-->", comment);
            }
            _ => {}
        }
        TokenSinkResult::Continue
    }
}

pub fn parse(html: &str) {
    let mut input = BufferQueue::default();
    input.push_back(StrTendril::from(html));

    let tokenizer = Tokenizer::new(Printer, TokenizerOpts::default());
    let _ = tokenizer.feed(&mut input);
    tokenizer.end();
}
