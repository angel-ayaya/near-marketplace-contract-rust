use near_sdk::store::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, NearToken};
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

    #[payable]
    pub fn buy_product(&mut self, product_id: &String) {
        match self.listed_products.get(product_id).cloned() {
            Some(ref mut product) => {

                let price_yoctonear: u128 = product.price.parse().expect("The product's price is not valid");

                let deposit = env::attached_deposit();
                let price_token = NearToken::from_yoctonear(price_yoctonear);
                assert_eq!(deposit, price_token, "Attached deposit should be equal to the product's price.");

                let owner = &product.owner.as_str();
                Promise::new(owner.parse().unwrap()).transfer(NearToken::from_yoctonear(price_yoctonear));

                product.increment_sold_amount();
                self.listed_products.insert(product.id.clone(), product.clone());
            },
            _ => env::panic_str("product not found"),
        } 

}
}