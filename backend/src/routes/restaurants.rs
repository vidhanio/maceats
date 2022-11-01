use maceats::{CoffeeBrand, FoodType};

super::routes! {
    restaurants("restaurants") {
        all: warp::path!(),
        open_now: warp::path!("open-now"),
        by_food_type: warp::path!("food-type" / FoodType),
        by_coffee_brand: warp::path!("coffee-brand" / CoffeeBrand),
    }
}
