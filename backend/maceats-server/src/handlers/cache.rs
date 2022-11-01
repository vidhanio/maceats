use std::collections::HashMap;

use maceats::{CoffeeBrand, Error, FoodType, Location, Restaurant, Result};

#[derive(Default)]
pub struct Cache {
    restaurants_all: Option<Vec<Restaurant>>,
    restaurants_food_type: HashMap<FoodType, Vec<Restaurant>>,
    restaurants_coffee_brand: HashMap<CoffeeBrand, Vec<Restaurant>>,

    locations_all: Option<Vec<Location>>,
    location_restaurants: HashMap<Location, Vec<Restaurant>>,
}

impl Cache {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn invalidate(&mut self) {
        *self = Default::default();
    }

    pub async fn restaurants_all(&mut self) -> Result<Vec<Restaurant>> {
        if self.restaurants_all.is_none() {
            self.restaurants_all = Some(Restaurant::all().await?);
        }

        self.restaurants_all
            .clone()
            .ok_or(Error::Misc("cache error"))
    }

    pub async fn restaurants_food_type(&mut self, food_type: FoodType) -> Result<Vec<Restaurant>> {
        if !self.restaurants_food_type.contains_key(&food_type) {
            let restaurants = self
                .restaurants_all()
                .await?
                .iter()
                .filter(|&r| r.tags.contains(&food_type))
                .map(|r| r.clone())
                .collect();
            self.restaurants_food_type.insert(food_type, restaurants);
        }

        self.restaurants_food_type
            .get(&food_type)
            .cloned()
            .ok_or(Error::Misc("cache error"))
    }

    pub async fn restaurants_coffee_brand(
        &mut self,
        coffee_brand: CoffeeBrand,
    ) -> Result<Vec<Restaurant>> {
        if !self.restaurants_coffee_brand.contains_key(&coffee_brand) {
            self.restaurants_coffee_brand
                .insert(coffee_brand, coffee_brand.restaurants().await?);
        }

        self.restaurants_coffee_brand
            .get(&coffee_brand)
            .cloned()
            .ok_or(Error::Misc("cache error"))
    }

    pub async fn locations_all(&mut self) -> Result<Vec<Location>> {
        if self.locations_all.is_none() {
            self.locations_all = Some(Location::all().await?);
        }

        self.locations_all.clone().ok_or(Error::Misc("cache error"))
    }

    pub async fn location_restaurants(&mut self, location: Location) -> Result<Vec<Restaurant>> {
        if !self.location_restaurants.contains_key(&location) {
            let restaurants = self
                .restaurants_all()
                .await?
                .iter()
                .filter(|&r| r.location == location)
                .map(|r| r.clone())
                .collect();
            self.location_restaurants
                .insert(location.clone(), restaurants);
        }

        self.location_restaurants
            .get(&location)
            .cloned()
            .ok_or(Error::Misc("cache error"))
    }
}
