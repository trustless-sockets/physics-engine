use thiserror::Error;

use crate::operand::{
    BinOpOperand, DerefOperand, DerefOrImmediate, DoubleDerefOperand, ImmediateOperand, Register,
    ResOperand,
};

#[cfg(test)]
#[path = "ap_change_test.rs"]
mod test;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApChange {
    Known(i16),
    Unknown,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ApChangeError {
    #[error("Unknown ap change")]
    UnknownApChange,
    #[error("Offset overflow")]
    OffsetOverflow,
}

pub trait ApplyApChange: Sized {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError>;
}

impl ApplyApChange for ResOperand {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError> {
        Ok(match self {
            ResOperand::Deref(operand) => ResOperand::Deref(operand.apply_ap_change(ap_change)?),
            ResOperand::DoubleDeref(operand) => {
                ResOperand::DoubleDeref(operand.apply_ap_change(ap_change)?)
            }
            ResOperand::Immediate(operand) => {
                ResOperand::Immediate(operand.apply_ap_change(ap_change)?)
            }
            ResOperand::BinOp(operand) => ResOperand::BinOp(operand.apply_ap_change(ap_change)?),
        })
    }
}

impl ApplyApChange for DerefOperand {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError> {
        match self {
            DerefOperand { register: Register::AP, offset } => match ap_change {
                ApChange::Unknown => Err(ApChangeError::UnknownApChange),
                ApChange::Known(ap_change) => Ok(DerefOperand {
                    register: Register::AP,
                    offset: offset.checked_sub(ap_change).ok_or(ApChangeError::OffsetOverflow)?,
                }),
            },
            DerefOperand { register: Register::FP, offset: _ } => Ok(self),
        }
    }
}

impl ApplyApChange for DoubleDerefOperand {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError> {
        Ok(DoubleDerefOperand { inner_deref: self.inner_deref.apply_ap_change(ap_change)? })
    }
}

impl ApplyApChange for ImmediateOperand {
    fn apply_ap_change(self, _ap_change: ApChange) -> Result<Self, ApChangeError> {
        Ok(self)
    }
}

impl ApplyApChange for DerefOrImmediate {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError> {
        Ok(match self {
            DerefOrImmediate::Deref(operand) => {
                DerefOrImmediate::Deref(operand.apply_ap_change(ap_change)?)
            }
            DerefOrImmediate::Immediate(operand) => {
                DerefOrImmediate::Immediate(operand.apply_ap_change(ap_change)?)
            }
        })
    }
}

impl ApplyApChange for BinOpOperand {
    fn apply_ap_change(self, ap_change: ApChange) -> Result<Self, ApChangeError> {
        Ok(BinOpOperand {
            op: self.op,
            a: self.a.apply_ap_change(ap_change)?,
            b: self.b.apply_ap_change(ap_change)?,
        })
    }
}