import { Restaurant, Times } from "../types";

type Props = {
  restaurant: Restaurant;
};

export function TimesTable({ schedule }: { schedule: Record<string, Times> }) {
  return (
    <table className="table-auto">
      <thead>
        <tr>
          <th className="border px-4 py-2">Day</th>
          <th className="border px-4 py-2">Time</th>
        </tr>
      </thead>
      <tbody>
        {Object.entries(schedule).map(([day, times]) => (
          <tr key={day}>
            <td className="border px-4 py-2">
              <time dateTime={day}>{day}</time>
            </td>
            <td className="border px-4 py-2">
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

export default function RestaurantCard({ restaurant }: Props) {
  return (
    <div className="flex flex-col text-center items-center justify-center">
      <h1 className="text-2xl font-bold">{restaurant.name}</h1>
      <a className="text-blue-500" href={restaurant.location.url}>
        {restaurant.location.name}
      </a>
      {restaurant.schedule && <TimesTable schedule={restaurant.schedule} />}
    </div>
  );
}
