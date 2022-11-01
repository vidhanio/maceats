import { Response, Location } from "../types";
import LocationCard from "./location-card";

type Props = {
  promise: Promise<Response<Location[]>>;
};

export default async function LocationList({ promise }: Props) {
  const locations = await promise;

  if (locations.error) throw new Error(locations.error);

  return (
    <div className="flex flex-row flex-wrap items-center justify-center gap-8 p-8">
      {locations.data?.map((location) => (
        <LocationCard key={location.slug} location={location} />
      ))}
    </div>
  );
}
