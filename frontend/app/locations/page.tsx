import { Suspense } from "react";
import LocationList from "../../components/location-list";
import { get, Location } from "../../types";

export default async function Page() {
  const promise = get<Location[]>(`/locations`);

  return (
    <>
      <h1 className="text-4xl font-bold">Locations</h1>
      <Suspense fallback={<div>Loading...</div>}>
        <LocationList promise={promise} />
      </Suspense>
    </>
  );
}
