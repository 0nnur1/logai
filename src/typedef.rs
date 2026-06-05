#[derive(Debug, Clone, Copy)]
pub struct Bitfield {
    pub(crate) data: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct PayloadPart {
    pub(crate) data: Bitfield,
}
#[derive(Debug, Clone, Copy)]
pub struct RadixNode {
    pub(crate) children: Payload,
    pub(crate) payload: Payload,
}
#[derive(Debug, Clone, Copy)]
pub struct Payload {
    // Index 0 = false (right), Index 1 = true (left)
    pub(crate) parts: [PayloadPart; 2],
}
#[derive(Debug, Clone)]
pub struct Radix {
    pub(crate) nodes: Vec<RadixNode>,
}
