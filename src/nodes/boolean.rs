pub struct And<'a> {
    in_a: Box<dyn Iterator<Item = bool> + 'a>,
    in_b: Box<dyn Iterator<Item = bool> + 'a>,
}

impl<'a> Node for And<'a> {}

impl<'a> And<'a> {
    pub fn new(in_a: Box<impl Iterator<Item = bool> + 'a>, in_b: Box<impl Iterator<Item = bool> + 'a>) -> Self {
        And { in_a, in_b }
    }
}

impl<'a> Iterator for And<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.in_a.next().unwrap_or(false);
        let b = self.in_b.next().unwrap_or(false);
        Some(a & b)
    }
}

pub struct Not<'a> {
    input: Box<dyn Iterator<Item = bool> + 'a>,
}

impl<'a> Node for Not<'a> {}

impl<'a> Not<'a> {
    pub fn new(input: Box<dyn Iterator<Item = bool> + 'a>) -> Self {
        Not { input }
    }
}

impl<'a> Iterator for Not<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        Some(!self.input.next().unwrap_or(false))
    }
}
