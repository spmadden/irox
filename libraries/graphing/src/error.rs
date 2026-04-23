// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::fmt::Formatter;
use irox_tools::sync::PoisonedError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphErrorType {
    PoisonedLock,
    NegativeCost,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GraphError {
    pub error_type: GraphErrorType,
    pub message: String,
}
impl core::fmt::Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "GraphError({:?}): {}", self.error_type, self.message)
    }
}

impl core::error::Error for GraphError {}

impl From<PoisonedError> for GraphError {
    fn from(value: PoisonedError) -> Self {
        Self {
            error_type: GraphErrorType::PoisonedLock,
            message: value.to_string(),
        }
    }
}

impl GraphError {
    pub fn negative_cost<T>(message: String) -> Result<T, GraphError> {
        Err(GraphError {
            error_type: GraphErrorType::NegativeCost,
            message,
        })
    }
}
