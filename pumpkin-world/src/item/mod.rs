mod item_registry;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
/// Item Rarity
pub enum Rarity {
    Common,
    UnCommon,
    Rare,
    Epic,
}

#[derive(Clone, Copy)]
pub struct Item {
    item_count: u32,
    // This ID is the numerical protocol ID, not the usual minecraft::block ID.
    item_id: u32,
    // TODO: Add Item Components
}
