import Image from "next/image";
import RestaurantCard from "../components/restaurant";
import { Response, Restaurant } from "../types";

export default async function Page() {
  const restaurants: Response<Restaurant[]> = await fetch(
    "http://localhost:8080/locations/student-centre"
  ).then((res) => res.json());

  return (
    <div className="flex flex-col gap-4 items-center justify-center min-h-screen py-2">
      {restaurants.data?.map((restaurant) => (
        <RestaurantCard key={restaurant.name} restaurant={restaurant} />
      ))}
    </div>
  );
}
