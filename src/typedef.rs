pub struct RadixNode {
    pub left_child: Bitfield,
    pub right_child: Bitfield,
    pub payload: Payload,
}

#[derive(Debug)]
pub struct NullExceptionError(pub &'static str);

pub struct Bitfield {
    pub(crate) data: u64,
}

pub struct Payload {
    // Index 0 = false (right), Index 1 = true (left)
    pub(crate) parts: [PayloadPart; 2],
}

pub struct PayloadPart {
    pub data: Bitfield,
}
