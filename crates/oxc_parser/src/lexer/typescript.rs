use super::{Kind, Lexer, Token};

impl Lexer<'_> {
    /// Re-tokenize '<<' or '<=' or '<<=' to '<'
    pub(crate) fn re_lex_as_typescript_l_angle(&mut self, kind: Kind) -> Token {
        let offset = match kind {
            Kind::ShiftLeft | Kind::LtEq => 2,
            Kind::ShiftLeftEq => 3,
            _ => unreachable!(),
        };
        self.token.set_start(self.offset() - offset);
        self.source.back(offset as usize - 1);
        let kind = Kind::LAngle;
        self.finish_next(kind)
    }

    /// Re-tokenize '>>' and '>>>' to '>'
    pub(crate) fn re_lex_as_typescript_r_angle(&mut self, kind: Kind) -> Token {
        let offset = match kind {
            Kind::ShiftRight => 2,
            Kind::ShiftRight3 => 3,
            _ => unreachable!(),
        };
        self.token.set_start(self.offset() - offset);
        self.source.back(offset as usize - 1);
        let kind = Kind::RAngle;
        self.finish_next(kind)
    }
}
