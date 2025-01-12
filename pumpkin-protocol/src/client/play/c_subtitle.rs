use pumpkin_macros::packet;
use pumpkin_text::TextComponent;
use serde::Serialize;

#[derive(Serialize)]
#[packet(0x63)]
pub struct CSubtitle<'a> {
    subtitle: TextComponent<'a>,
}

impl<'a> CSubtitle<'a> {
    pub fn new(subtitle: TextComponent<'a>) -> Self {
        Self { subtitle }
    }
}
