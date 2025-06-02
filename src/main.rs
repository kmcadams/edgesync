mod device;
mod retail_store;
mod types;

use std::iter;

use retail_store::RetailStore;
use types::InventoryItem;

use crate::device::Device;

#[tokio::main]
async fn main() {
    let item_a = InventoryItem::new("Shoes", "123", 10, 50.0, "Tank City");
    let item_b = InventoryItem::new("Tanks", "987", 9, 750.0, "Tank City");
    let item_c = InventoryItem::new("Jets", "14a", 20, 550.0, "Tank City");

    let retail_a = RetailStore::with_inventory("Tank City", vec![item_a, item_b, item_c]);

    let item_d = InventoryItem::new("Shoes", "123", 10, 50.0, "Fort Worth");
    let item_e = InventoryItem::new("Tanks", "987", 9, 750.0, "Fort Worth");

    let mut retail_b = RetailStore::with_inventory("Fort Worth", vec![item_d, item_e]);

    retail_b.sell_item("123", 1);

    println!("Retail B before sync: {:?}", retail_b.inventory.get("123"));

    let mut retail_a = retail_a;
    retail_a.sync_with(&retail_b);

    println!("Retail A after sync: {:?}", retail_a.inventory.get("123"));
}

fn setup_and_merge_device() {
    let mut device_a = Device::new("A");
    let mut device_b = Device::new("B");

    device_a.insert("temperature", "20C");
    device_a.insert("humidity", "30");

    device_b.insert("temperature", "22C");
    device_b.insert("humidity", "29%");
    device_b.insert("battery", "80%");

    println!("\n--- Before merge ---");
    println!("Device A: {:?}", device_a.data);
    println!("Device B: {:?}", device_b.data);

    device_a.merge(&device_b.data);

    println!("\n--- After merge ---");
    println!("Device A: {:?}", device_a.data);
}
