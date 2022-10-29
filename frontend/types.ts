export type Location = {
  name: string;
  url: string;
};

export type Restaurant = {
  name: string;
  location: Location;
  location_details?: string;
  location_phone?: string;
  schedule?: Record<string, Times>;
  tags: FoodType[];
};

export type Times = { open: { from: string; to: string }[] } | "closed";

export type FoodType =
  | "breakfast"
  | "coffee"
  | "convenience"
  | "dessert"
  | "gluten-free"
  | "grill"
  | "halal"
  | "kosher"
  | "noodles"
  | "pasta"
  | "pizza"
  | "sandwiches"
  | "snacks"
  | "soup"
  | "sushi"
  | "vegetarian";

export type CoffeeBrand =
  | "marley"
  | "rejuvenate"
  | "starbucks"
  | "tim-hortons"
  | "williams";

export type Response<T> = {
  data?: T;
  error?: string;
};
