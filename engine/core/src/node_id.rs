use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

impl NodeId {
    pub const NONE: Self = Self(u32::MAX);

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn is_none(self) -> bool {
        self.0 == u32::MAX
    }
}

impl From<u32> for NodeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
