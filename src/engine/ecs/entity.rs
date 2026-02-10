#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

pub struct EntityAllocator {
    next: u32,
    free: Vec<u32>,
}

impl EntityAllocator {
    pub const fn new() -> Self {
        Self {
            next: 0,
            free: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> EntityId {
        if let Some(id) = self.free.pop() {
            EntityId(id)
        } else {
            let id = self.next;
            self.next = self.next.saturating_add(1);
            EntityId(id)
        }
    }

    pub fn free(&mut self, entity: EntityId) {
        self.free.push(entity.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_sequential_ids_when_no_free() {
        let mut allocator = EntityAllocator::new();
        let first = allocator.alloc();
        let second = allocator.alloc();
        let third = allocator.alloc();

        assert_eq!(first.0, 0);
        assert_eq!(second.0, 1);
        assert_eq!(third.0, 2);
    }

    #[test]
    fn reuses_freed_ids_before_growing() {
        let mut allocator = EntityAllocator::new();
        let first = allocator.alloc();
        let second = allocator.alloc();

        allocator.free(first);

        let reused = allocator.alloc();
        let next = allocator.alloc();

        assert_eq!(reused.0, first.0);
        assert_eq!(next.0, 2);
        assert_eq!(second.0, 1);
    }
}
