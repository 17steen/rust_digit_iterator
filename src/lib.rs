pub struct DigitIterator<const N: u64> {
    curr: u64,
}

impl<const N: u64> Iterator for DigitIterator<N> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.curr == 0 {
            return None;
        }
        let digit = self.curr % N;
        self.curr /= N;
        Some(digit)
    }
}

pub trait IntoDigitIterator {
    fn into_digits<const N: u64>(&self) -> DigitIterator<N>;
}

impl IntoDigitIterator for u64 {
    fn into_digits<const N: u64>(&self) -> DigitIterator<N> {
        DigitIterator { curr: *self }
    }
}
