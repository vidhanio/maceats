use maceats::{CoffeeBrand, FoodType, Restaurant};

super::handlers! {
    all: () => Restaurant::all(),
    open_now: () => Restaurant::open_now(),
    by_food_type: (food: FoodType) => food.restaurants(),
    by_coffee_brand: (coffee: CoffeeBrand) => coffee.restaurants(),
}
