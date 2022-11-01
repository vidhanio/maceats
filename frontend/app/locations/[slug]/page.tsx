import { get, Location, Response, Restaurant } from "../../../types";
import RestaurantList from "../../../components/restaurant-list";
import { Suspense } from "react";

type Params = {
  params: {
    slug: string;
  };
};

export default async function Page({ params: { slug } }: Params) {
  const locations = await get<Location[]>("/locations");

  if (locations.error) throw new Error(locations.error);

  const location = locations.data?.find((location) => location.slug === slug);

  const promise = get<Restaurant[]>(`/locations/${slug}`);

  return (
    <div className="flex flex-col items-center justify-center gap-8 p-8 text-center">
      <h1 className="text-4xl font-bold">Restaurants - {location?.name}</h1>
      <Suspense fallback={<div>Loading...</div>}>
        <RestaurantList promise={promise} />
      </Suspense>
    </div>
  );
}
