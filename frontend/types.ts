export type Location = {
  name: string;
  slug: string;
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

export const API_URL =
  process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export async function get<T>(path: string): Promise<Response<T>> {
  return fetch(`${API_URL}${path}`, {
    cache: "no-store",
  }).then((res) => res.json());
}
