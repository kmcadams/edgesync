use std::collections::HashMap;

use crate::types::InventoryItem;
use vclock::VClock;

pub struct RetailStore {
    pub id: String,
    pub clock: VClock<String>,
    pub inventory: HashMap<String, InventoryItem>,
}

impl RetailStore {
    pub fn with_inventory(id: &str, items: Vec<InventoryItem>) -> Self {
        let inventory = items
            .into_iter()
            .map(|item| (item.sku.clone(), item))
            .collect::<HashMap<_, _>>();

        Self {
            id: id.to_string(),
            clock: VClock::new(id.to_string()),
            inventory,
        }
    }
    pub fn sync_with(&mut self, other: &RetailStore) {
        for (sku, other_item) in &other.inventory {
            match self.inventory.get_mut(sku) {
                Some(local_item) => {
                    let merged = local_item.merge(other_item);
                    *local_item = merged;
                }
                None => {
                    self.inventory.insert(sku.clone(), other_item.clone());
                }
            }
        }

        self.clock.merge(&other.clock);
    }

    pub fn sell_item(&mut self, sku: &str, quantity: u32) {
        if let Some(item) = self.inventory.get_mut(sku) {
            item.update_quantity(-(quantity as i32), &self.id);
        }
    }

    pub fn change_price(&mut self, sku: &str, new_price: f32) {
        if let Some(item) = self.inventory.get_mut(sku) {
            item.update_price(new_price, &self.id);
        }
    }
}
