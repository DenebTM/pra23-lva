use crate::{
    block::{Block, TestBlock},
    expression::{AExp, BExp, Label, Variable},
    statement::Statement,
};

#[derive(Clone, Debug)]
pub struct StatementBuilder<'a> {
    contents: Statement<'a>,
    next_label: Label,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(first_label: Label) -> Self {
        Self {
            contents: Statement::Empty,
            next_label: first_label,
        }
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::assignment(self.next_label, var, expr));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn skip(self) -> Self {
        let new_stmt = Statement::Atom(Block::skip(self.next_label));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::test(self.next_label, expr));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn if_then(self, test: BExp<'a>) -> IfThenBuilder<'a> {
        IfThenBuilder::new(test, self)
    }

    pub fn while_(self, test: BExp<'a>) -> WhileBuilder<'a> {
        WhileBuilder::new(test, self)
    }

    pub fn end(self) -> Statement<'a> {
        self.contents
    }

    fn append(self, stmt: Statement<'a>, next_label: Label) -> Self {
        Self {
            contents: append(self.contents, stmt),
            next_label,
        }
    }
}

pub struct IfThenBuilder<'a> {
    test: BExp<'a>,
    then_builder: StatementBuilder<'a>,
    parent: StatementBuilder<'a>,
}
impl<'a> IfThenBuilder<'a> {
    fn new(test: BExp<'a>, super_builder: StatementBuilder<'a>) -> Self {
        let mut inst = Self {
            test,
            then_builder: StatementBuilder::new(0),
            parent: super_builder,
        };
        inst.then_builder.next_label = inst.parent.next_label;

        inst
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        Self {
            test: self.test,
            then_builder: self.then_builder.assignment(var, expr),
            parent: self.parent,
        }
    }

    pub fn skip(self) -> Self {
        Self {
            test: self.test,
            then_builder: self.then_builder.skip(),
            parent: self.parent,
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        Self {
            test: self.test,
            then_builder: self.then_builder.test(expr),
            parent: self.parent,
        }
    }

    pub fn else_(self) -> ElseBuilder<'a> {
        ElseBuilder::new(self)
    }
}

pub struct ElseBuilder<'a> {
    if_builder: IfThenBuilder<'a>,
    else_builder: StatementBuilder<'a>,
}
impl<'a> ElseBuilder<'a> {
    fn new(if_builder: IfThenBuilder<'a>) -> Self {
        let mut inst = Self {
            if_builder,
            else_builder: StatementBuilder::new(0),
        };
        inst.else_builder.next_label = inst.if_builder.then_builder.next_label;

        inst
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        Self {
            if_builder: self.if_builder,
            else_builder: self.else_builder.assignment(var, expr),
        }
    }

    pub fn skip(self) -> Self {
        Self {
            if_builder: self.if_builder,
            else_builder: self.else_builder.skip(),
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        Self {
            if_builder: self.if_builder,
            else_builder: self.else_builder.test(expr),
        }
    }

    pub fn end(self) -> StatementBuilder<'a> {
        let test_label = self.if_builder.parent.next_label;
        let next_label = self.else_builder.next_label;

        self.if_builder.parent.append(
            Statement::IfThenElse(
                TestBlock {
                    label: test_label,
                    expr: self.if_builder.test,
                },
                Box::new(self.if_builder.then_builder.end()),
                Box::new(self.else_builder.end()),
            ),
            next_label,
        )
    }
}

pub struct WhileBuilder<'a> {
    test: BExp<'a>,
    while_builder: StatementBuilder<'a>,
    parent: StatementBuilder<'a>,
}
impl<'a> WhileBuilder<'a> {
    fn new(test: BExp<'a>, super_builder: StatementBuilder<'a>) -> Self {
        let mut inst = Self {
            test,
            while_builder: StatementBuilder::new(0),
            parent: super_builder,
        };
        inst.while_builder.next_label = inst.parent.next_label;

        inst
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        Self {
            test: self.test,
            while_builder: self.while_builder.assignment(var, expr),
            parent: self.parent,
        }
    }

    pub fn skip(self) -> Self {
        Self {
            test: self.test,
            while_builder: self.while_builder.skip(),
            parent: self.parent,
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        Self {
            test: self.test,
            while_builder: self.while_builder.test(expr),
            parent: self.parent,
        }
    }

    pub fn end(self) -> StatementBuilder<'a> {
        let test_label = self.parent.next_label;
        let next_label = self.while_builder.next_label;

        self.parent.append(
            Statement::While(
                TestBlock {
                    label: test_label,
                    expr: self.test,
                },
                Box::new(self.while_builder.end()),
            ),
            next_label,
        )
    }
}

fn append<'a>(stmt: Statement<'a>, next: Statement<'a>) -> Statement<'a> {
    match stmt {
        Statement::Empty => next,
        Statement::Composition(stmt1, stmt2) => {
            Statement::Composition(stmt1, Box::new(append(*stmt2, next)))
        }
        other_first => Statement::Composition(Box::new(other_first), Box::new(next)),
    }
}
