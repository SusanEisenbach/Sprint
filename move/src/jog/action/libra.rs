use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const DEPENDENCIES: &[&str] = &["0x0.LibraAccount", "0x0.LibraCoin"];

pub struct Lock {
    amount: u64,
    locked: Rc<Variable>,
}

impl Lock {
    pub fn new(amount: u64) -> Self {
        Lock {
            amount,
            locked: Rc::new(Variable {
                // TODO: generate name to avoid clash.
                name: "locked",
                type_name: "LibraCoin.T",
                default: Some("LibraCoin.zero()"),
            }),
        }
    }
}

impl Action for Lock {
    fn dependencies(&self) -> &'static [&'static str] {
        DEPENDENCIES
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![self.locked.clone()]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Lock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraCoin.deposit(&mut move(contract_ref).{}, LibraAccount.withdraw_from_sender({}));",
            self.locked.name, self.amount
        )
    }
}

pub struct Unlock {
    locked: Rc<Variable>,
}

impl Unlock {
    pub fn new(action: &Lock) -> Self {
        Unlock {
            locked: action.locked.clone(),
        }
    }
}

impl Action for Unlock {
    fn dependencies(&self) -> &'static [&'static str] {
        DEPENDENCIES
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Unlock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraAccount.deposit(copy(counterparty), move({}));",
            self.locked.name
        )
    }
}
