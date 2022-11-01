use maceats::{CoffeeBrand, FoodType, Restaurant};

use super::CACHE;

super::handlers! {
    all: () => CACHE.lock().await.restaurants_all(),
    open_now: () => Restaurant::open_now(),
    by_food_type: (food: FoodType) => CACHE.lock().await.restaurants_food_type(food),
    by_coffee_brand: (coffee: CoffeeBrand) => CACHE.lock().await.restaurants_coffee_brand(coffee),
}
