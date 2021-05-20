pub struct DigitIterator<const BASE: u64> {
    curr: u64,
}

impl<const BASE: u64> Iterator for DigitIterator<BASE> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.curr == 0 {
            return None;
        }
        let digit = self.curr % BASE;
        self.curr /= BASE;
        Some(digit)
    }
}

pub trait AsDigitIterator {
    fn as_digits<const BASE: u64>(&self) -> DigitIterator<BASE>;
}

impl AsDigitIterator for u64 {
    fn as_digits<const BASE: u64>(&self) -> DigitIterator<BASE> {
        DigitIterator { curr: *self }
    }
}
