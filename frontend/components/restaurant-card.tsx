import Link from "next/link";
import { FoodType, Restaurant, Times } from "../types";
import Tag from "./tag";

type Props = {
  restaurant: Restaurant;
  tag?: FoodType;
};

export function ScheduleTable({
  schedule,
  className,
}: {
  schedule: Record<string, Times>;
  className?: string;
}) {
  return (
    <table className={`${className} table-auto`}>
      <thead>
        <tr>
          <th className="px-4 py-2 border border-gray-500">Day</th>
          <th className="px-4 py-2 border border-gray-500">Time</th>
        </tr>
      </thead>
      <tbody>
        {Object.entries(schedule).map(([day, times]) => (
          <tr key={day}>
            <td className="px-4 py-2 border border-gray-500">
              <time dateTime={day}>{day}</time>
            </td>
            <td className="px-4 py-2 border border-gray-500">
              {times == "closed"
                ? "Closed"
                : times.open
                    .map(({ from, to }) => `${from} - ${to}`)
                    .join(", ")}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default function RestaurantCard({ restaurant, tag }: Props) {
  return (
    <div className="flex flex-col items-center justify-center w-full gap-4 p-8 text-center text-gray-800 bg-gray-200 shadow-xl lg:flex-row lg:w-4/5 rounded-xl dark:bg-gray-800 dark:text-gray-200">
      <div className="flex flex-col items-center justify-center w-full gap-4">
        <h1 className="text-2xl font-bold">{restaurant.name}</h1>
        <Link
          className="text-blue-500"
          href={`/locations/${restaurant.location.slug}`}
        >
          {restaurant.location.name}
        </Link>
        {restaurant.location_details && (
          <p className="text-sm">{restaurant.location_details}</p>
        )}
        {restaurant.location_phone && (
          <p className="text-sm">{restaurant.location_phone}</p>
        )}
        {restaurant.tags && (
          <div className="flex flex-row flex-wrap items-center justify-center gap-2 lg:w-1/2">
            {restaurant.tags.map((t) => (
              <Tag key={t} tag={t} selected={tag ? tag == t : false} />
            ))}
          </div>
        )}
      </div>
      {restaurant.schedule && (
        <ScheduleTable className="w-full" schedule={restaurant.schedule} />
      )}
    </div>
  );
}
