use std::{cmp::PartialOrd, fmt, str::FromStr};

use anyhow::Error;
use graph::impl_slog_value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct NodeCapabilities {}

impl FromStr for NodeCapabilities {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(NodeCapabilities {})
    }
}

impl fmt::Display for NodeCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{{CHAIN_NAME}}")
    }
}

impl_slog_value!(NodeCapabilities, "{}");

