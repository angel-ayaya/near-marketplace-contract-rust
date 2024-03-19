use near_sdk::store::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::serde::{Serialize, Deserialize};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    listed_products: UnorderedMap<String, Product>
    
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PanicOnDefault)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
    owner: AccountId,
    sold: u32
}


#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String
}

impl Product {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            description: payload.description,
            name: payload.name,
            location: payload.location,
            price: payload.price,
            sold: 0,
            image: payload.image,
            owner: env::signer_account_id()
        }
    }

    pub fn increment_sold_amount(&mut self) {
        self.sold = self.sold + 1;
    }
}

#[near_bindgen]
impl Marketplace {
    
    #[init]
    pub fn init() -> Self {
        Self {
            listed_products: UnorderedMap::new(b"listed_products".to_vec()),
        }
    }

    pub fn set_product(&mut self, payload: Payload) {
        let product = Product::from_payload(payload);
        self.listed_products.insert(product.id.clone(), product);

    }

    pub fn get_product(&self, id: &String) -> Option<Product> {
        self.listed_products.get(id).cloned()
    }

    pub fn get_products(&self) -> Vec<Product> {
        return self.listed_products.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>();
    }
}

