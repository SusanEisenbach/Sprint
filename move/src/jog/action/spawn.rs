use super::{
    super::{expression::Address, expression::Expression, variable::Variable},
    Action,
};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Spawn<'a> {
    context: Rc<Variable<'a>>,
    root: Expression<'a>,
}

impl<'a> Spawn<'a> {
    pub fn new(context: Rc<Variable<'a>>, root: Expression<'a>) -> Self {
        Spawn { context, root }
    }
}

impl Action for Spawn<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.context]
    }
}

impl Display for Spawn<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} = Context {{
                state: {},
                holder: {},
                counterparty: {},
                scale: *(&copy(context_ref).scale),
            }};",
            self.context.identifier(),
            self.root,
            Address::Holder,
            Address::Counterparty,
        )
    }
}

pub struct PushContext<'a> {
    context: Rc<Variable<'a>>,
}

impl<'a> PushContext<'a> {
    pub fn new(context: Rc<Variable<'a>>) -> Self {
        PushContext { context }
    }
}

impl Action for PushContext<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.context]
    }
}

impl Display for PushContext<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "Vector.push_back<Self.Context>(copy(contexts), move({}));",
            self.context.identifier()
        )
    }
}
