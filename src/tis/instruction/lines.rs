#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct Line;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug, Default)]
pub struct Lines<S> {
    lines: S,
}

impl<S: AsRef<[Line]>> Lines<S> {
    pub fn new(lines: S) -> Lines<S> {
        Lines { lines }
    }
    pub fn into_inner(self) -> S {
        self.lines
    }
}
impl<S: AsRef<[Line]>> AsRef<[Line]> for Lines<S> {
    fn as_ref(&self) -> &[Line] {
        self.lines.as_ref()
    }
}
