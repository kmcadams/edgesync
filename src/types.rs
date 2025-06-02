use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use vclock::VClock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub value: String,
    pub timestamp: VClock<String>,
}

pub type DataMap = std::collections::HashMap<String, DataEntry>;

pub trait CRDTField {
    fn get_clock(&self) -> &VClock<String>;
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone)]
pub struct CrdtValue<T> {
    pub value: T,
    pub clock: VClock<String>,
}

impl<T: Clone + PartialOrd> CRDTField for CrdtValue<T> {
    fn get_clock(&self) -> &VClock<String> {
        &self.clock
    }

    fn merge(&self, other: &Self) -> Self {
        match self.clock.partial_cmp(&other.clock) {
            Some(Ordering::Less) => other.clone(),
            Some(Ordering::Greater) => self.clone(),
            None => {
                if self.value <= other.value {
                    self.clone()
                } else {
                    other.clone()
                }
            }
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub sku: String,
    pub quantity: CrdtValue<u32>,
    pub price: CrdtValue<f32>,
}

impl InventoryItem {
    pub fn new(name: &str, sku: &str, quantity: u32, price: f32, store_id: &str) -> Self {
        let mut clock = VClock::new(store_id.to_string());
        clock.incr(&store_id.to_string());

        Self {
            name: name.to_string(),
            sku: sku.to_string(),
            quantity: CrdtValue {
                value: quantity,
                clock: clock.clone(),
            },
            price: CrdtValue {
                value: price,
                clock,
            },
        }
    }
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            name: self.name.clone(),
            sku: self.sku.clone(),
            quantity: self.quantity.merge(&other.quantity),
            price: self.price.merge(&other.price),
        }
    }

    pub fn update_quantity(&mut self, delta: i32, store_id: &str) {
        let new_value = (self.quantity.value as i32 + delta).max(0) as u32;
        self.quantity.value = new_value;
        self.quantity.clock.incr(&store_id.to_string());
    }

    pub fn update_price(&mut self, new_price: f32, store_id: &str) {
        self.price.value = new_price;
        self.price.clock.incr(&store_id.to_string());
    }
}
