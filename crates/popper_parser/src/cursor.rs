use std::fmt::Debug;
use popper_ast::Span;
use crate::error::Error;
use crate::expect::Expect;


trait Item: PartialEq + Clone + Debug {}
impl<T> Item for T where T: PartialEq + Clone + Debug {}

trait It<T: Item>: Iterator<Item = T> + Clone {}
impl<T, I> It<T> for I where I: Iterator<Item = T> + Clone {}

#[derive(Clone)]
pub struct Cursor<I: It<Item>> {
    pub iter: I,
    pub pos: usize,
    pub rec: usize
}

impl<I> Cursor<I>
where   I: Iterator + Clone,
        I::Item: PartialEq + Clone
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            rec: 0,
        }
    }

    pub fn from(&mut self, cursor: Cursor<I>) {
        self.iter = cursor.iter;
        self.pos = cursor.pos;
        self.rec = cursor.rec;
    }

    pub fn remove(&mut self, n: usize) -> Option<I::Item> {
        if let Some(e) = self.pos.checked_sub(1) {
            self.pos = e;
        }
        self.iter.nth(n)
    }

    pub fn find(&mut self, e: I::Item) -> Option<usize> {
        self.iter.clone().position(|e2| e == e2)
    }

    pub fn filter(&mut self, mut f: impl FnMut(&I::Item) -> bool) {
        let it = self.iter.clone();
        for (i, e) in it.enumerate() {
            if ! f(&e) {

                self.remove(i);
            }
        }

    }

    pub fn start_recording(&mut self) {
        self.rec = self.pos;
    }

    pub fn end_recording(&mut self) -> Span {
        Span::new(self.rec, self.pos)
    }

    pub fn peek(&mut self) -> Option<I::Item> {
        self.iter.clone().peekable().peek().cloned()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<I::Item> {
        self.pos += 1;
        self.iter.next()
    }

    pub fn prev(&mut self) -> Option<I::Item> {
        self.pos -= 1;
        self.iter.clone().nth(self.pos)
    }

    pub fn custom_prev(&mut self, n: usize) -> Option<I::Item> {
        self.pos -= n;
        self.iter.clone().nth(self.pos)
    }

    pub fn len(self) -> usize {
        self.iter.count()
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn collect(&mut self) -> Vec<I::Item> {
        self.iter.clone().collect()
    }

    pub fn expect<P: Expect<I>>(
        &mut self,
        expected: P
    ) -> Result<P, Error> {
        expected.expect(self)
    }

}