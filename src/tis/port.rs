use crate::tis::value::AtomicValueOption;
use std::sync::Arc;

struct ChannelInner(AtomicValueOption, AtomicValueOption);
pub struct Port {
    inner: Arc<ChannelInner>,
}
pub struct Ports {
    up: Option<Port>,
    down: Option<Port>,
    left: Option<Port>,
    right: Option<Port>,
}
