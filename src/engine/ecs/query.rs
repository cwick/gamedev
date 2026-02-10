use super::entity::EntityId;

pub struct QueryIter<'a, A, B> {
    index: usize,
    a: &'a Vec<Option<A>>,
    b: &'a Vec<Option<B>>,
}

impl<'a, A, B> QueryIter<'a, A, B> {
    pub fn new(a: &'a Vec<Option<A>>, b: &'a Vec<Option<B>>) -> Self {
        Self { index: 0, a, b }
    }
}

impl<'a, A, B> Iterator for QueryIter<'a, A, B> {
    type Item = (EntityId, &'a A, &'a B);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.a.len().min(self.b.len()) {
            let idx = self.index;
            self.index += 1;
            if let (Some(a), Some(b)) = (&self.a[idx], &self.b[idx]) {
                return Some((EntityId(idx as u32), a, b));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_only_pairs_in_index_order() {
        let a = vec![Some(10), None, Some(30), Some(40)];
        let b = vec![Some(1), Some(2), None, Some(4)];
        let ids: Vec<u32> = QueryIter::new(&a, &b).map(|(id, _, _)| id.0).collect();

        assert_eq!(ids, vec![0, 3]);
    }

    #[test]
    fn stops_at_shorter_storage_len() {
        let a = vec![Some(1), Some(2), Some(3)];
        let b = vec![Some(10)];
        let ids: Vec<u32> = QueryIter::new(&a, &b).map(|(id, _, _)| id.0).collect();

        assert_eq!(ids, vec![0]);
    }
}
