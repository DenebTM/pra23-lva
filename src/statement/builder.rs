use crate::{
    block::{Block, TestBlock},
    expression::{AExp, BExp, Label, Variable},
    statement::Statement,
};

#[derive(Clone, Debug)]
pub enum BuilderType<'a> {
    Plain,

    /// If(Test)
    If(TestBlock<'a>),

    /// Else(Test, If-Block)
    Else(TestBlock<'a>, Statement<'a>),

    /// While(Test)
    While(TestBlock<'a>),
}

#[derive(Clone, Debug)]
pub struct StatementBuilder<'a> {
    /// keep track of nesting
    parent: Option<Box<StatementBuilder<'a>>>,
    btype: BuilderType<'a>,
    contents: Statement<'a>,
    next_label: Label,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(first_label: Label) -> Self {
        Self {
            parent: None,
            btype: BuilderType::Plain,
            contents: Statement::Empty,
            next_label: first_label,
        }
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::assignment(self.next_label, var, expr));
        Self {
            parent: self.parent,
            btype: self.btype,
            contents: self.contents.append(new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn skip(self) -> Self {
        let new_stmt = Statement::Atom(Block::skip(self.next_label));
        Self {
            parent: self.parent,
            btype: self.btype,
            contents: self.contents.append(new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::test(self.next_label, expr));
        Self {
            parent: self.parent,
            btype: self.btype,
            contents: self.contents.append(new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn begin_if(self, test: BExp<'a>) -> Self {
        let next_label = self.next_label;
        Self {
            parent: Some(Box::new(self)),
            btype: BuilderType::If(TestBlock {
                label: next_label,
                expr: test,
            }),
            contents: Statement::Empty,
            next_label: next_label + 1,
        }
    }

    pub fn else_(self) -> Self {
        let next_label = self.next_label;
        let stmt1 = self.clone().end();
        match self.btype {
            BuilderType::If(test) => Self {
                parent: self.parent,
                btype: BuilderType::Else(test, stmt1),
                contents: Statement::Empty,
                next_label: next_label + 1,
            },
            _ => panic!("else called without prior if"),
        }
    }

    pub fn end_if(self) -> Self {
        let next_label = self.next_label;
        let stmt = self.clone().end();
        match self.btype {
            BuilderType::Else(test, stmt1) => self.parent.unwrap().append(
                Statement::IfThenElse(test, Box::new(stmt1), Box::new(stmt)),
                next_label,
            ),
            _ => panic!("end_if called without prior else"),
        }
    }

    pub fn begin_while(self, test: BExp<'a>) -> Self {
        let next_label = self.next_label;
        Self {
            parent: Some(Box::new(self)),
            btype: BuilderType::While(TestBlock {
                label: next_label,
                expr: test,
            }),
            contents: Statement::Empty,
            next_label: next_label + 1,
        }
    }

    pub fn end_while(self) -> Self {
        let next_label = self.next_label;
        let stmt = self.clone().end();
        match self.btype {
            BuilderType::While(test) => self
                .parent
                .unwrap()
                .append(Statement::While(test, Box::new(stmt)), next_label),
            _ => panic!("end_while called without prior while"),
        }
    }

    pub fn end(self) -> Statement<'a> {
        self.contents
    }

    fn append(self, stmt: Statement<'a>, next_label: Label) -> Self {
        Self {
            parent: self.parent,
            btype: self.btype,
            contents: self.contents.append(stmt),
            next_label,
        }
    }
}
