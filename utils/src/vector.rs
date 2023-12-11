pub trait UniquePermutations<T> {
    fn unique_permutations(self) -> Vec<(T, T)>
    where
        Self: Sized,
        T: Copy;
}

impl<T: Clone> UniquePermutations<T> for Vec<T> {
    fn unique_permutations(self) -> Vec<(T, T)>
    where
        Self: Sized,
        T: Copy,
    {
        self.iter()
            .enumerate()
            .take(self.len() - 1)
            .flat_map(|(index, e)| (index..self.len()).map(|next_index| (*e, self[next_index])))
            .collect::<Vec<_>>()
    }
}
