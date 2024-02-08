use bevy::prelude::*;
use std::ops::Index;

#[derive(Component, Clone, Debug)]
pub struct Inventory {
    items: Vec<Option<Entity>>,
}

impl Inventory {
    pub const CAPACITY: usize = 28;

    pub fn new() -> Self {
        Inventory {
            items: vec![None; Inventory::CAPACITY],
        }
    }

    pub fn add(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self.items.iter_mut().enumerate().find(|(_, b)| b.is_none()) {
            *e = Some(item);
            true
        } else {
            false
        }
    }

    pub fn take(&mut self, item: Entity) -> bool {
        if let Some((_, e)) = self
            .items
            .iter_mut()
            .enumerate()
            .find(|(_, b)| b.is_some() && b.unwrap() == item)
        {
            *e = None;
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        self.items.iter().all(|i| i.is_some())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for Inventory {
    type Output = Option<Entity>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
