import { Response } from "../types";
import RestaurantCard from "./restaurant-card";
import Button, { Props as ButtonProps } from "./button";

type Props = {
  promise: Promise<Response<ButtonProps[]>>;
};

export default async function ButtonList({ promise }: Props) {
  const buttons = await promise;

  if (buttons.error) throw new Error(buttons.error);

  return (
    <div className="flex flex-row flex-wrap items-center justify-center gap-8">
      {buttons.data?.map((button) => (
        <Button key={button.text} {...button} />
      ))}
    </div>
  );
}
