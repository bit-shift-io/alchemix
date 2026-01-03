use crate::chemistry::element::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Atom {
    pub element: Element,
}

impl Atom {
    pub fn new(element: Element) -> Self {
        Self { element }
    }
}
