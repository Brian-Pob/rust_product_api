use serde::{Deserialize, Serialize};

pub trait Product {
    fn set_id(&mut self, id: i32);
    fn set_name(&mut self, name: String);
    fn set_description(&mut self, description: String);
    fn set_price(&mut self, price: f64);
}

#[readonly::make]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base_Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
}

impl Product for Base_Product {
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
    fn set_price(&mut self, price: f64) {
        self.price = price;
    }
}

impl Base_Product {
    // new is the constructor
    pub fn new(id: i32, name: String, description: String, price: f64) -> Base_Product {
        Base_Product {
            id,
            name,
            description,
            price,
        }
    }

    pub fn get_data(&self) -> String {
        format!(
            "{} - {} - {} - {}",
            self.id, self.name, self.description, self.price
        )
    }
}

#[readonly::make]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight_Product {
    pub base: Base_Product,
    pub weight: f64,
}

impl Product for Weight_Product {
    fn set_id(&mut self, id: i32) {
        self.base.set_id(id);
    }
    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }
    fn set_description(&mut self, description: String) {
        self.base.set_description(description);
    }
    fn set_price(&mut self, price: f64) {
        self.base.set_price(price);
    }
}

impl Weight_Product {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        price: f64,
        weight: f64,
    ) -> Weight_Product {
        Weight_Product {
            base: Base_Product::new(id, name, description, price),
            weight,
        }
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }
}

#[readonly::make]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity_Product {
    pub base: Base_Product,
    pub quantity: i32,
}

impl Product for Quantity_Product {
    fn set_id(&mut self, id: i32) {
        self.base.set_id(id);
    }
    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }
    fn set_description(&mut self, description: String) {
        self.base.set_description(description);
    }
    fn set_price(&mut self, price: f64) {
        self.base.set_price(price);
    }
}

impl Quantity_Product {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        price: f64,
        quantity: i32,
    ) -> Quantity_Product {
        Quantity_Product {
            base: Base_Product::new(id, name, description, price),
            quantity,
        }
    }

    pub fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}
