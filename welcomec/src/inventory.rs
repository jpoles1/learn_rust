struct InventoryItem <'a> {
    name: &'a str,
    rarity: f32,
    price: u32
}
fn main() {
    let sword = InventoryItem{name: "Talking Sword", rarity: 0.2, price: 400};
    let mut inventory: Vec<InventoryItem> = Vec::with_capacity(25);
    inventory.push(sword);
    inventory.push(InventoryItem{name: "Potion", rarity: 0.6, price: 100});
    print_inventory(inventory)
}
fn print_inventory(inventory:Vec<InventoryItem>){
    for item in inventory{
        println!("\nItem: {}\nRarity: {}\nBase Price: {}", item.name, item.rarity, item.price)
    }
}
