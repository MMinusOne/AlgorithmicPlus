import { FaX } from "react-icons/fa6";

export function TopBar({
  handleClose,
  title,
}: {
  handleClose: () => void;
  title: string;
}) {
  return (
    <div className="w-full bg-base-300 p-1 flex justify-between">
      <span className="capitalize">{title}</span>
      <button
        onClick={handleClose}
        className="grid place-items-center cursor-pointer"
      >
        <FaX />
      </button>
    </div>
  );
}
  