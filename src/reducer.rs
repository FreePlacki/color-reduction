pub trait Reducer {
    fn reduce(&self, rgba: &[u8], width: usize, height: usize, num_colors: usize) -> Vec<u8>;
}
