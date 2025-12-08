pub trait Reducer {
    fn reduce(&self, rgba: &[u8], width: usize, height: usize) -> Vec<u8>;
}
