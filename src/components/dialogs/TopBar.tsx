import { FaArrowLeft, FaX } from "react-icons/fa6";

export function TopBar({
  handleClose,
  handleBack,
  title,
}: {
  handleClose: () => void;
  handleBack?: () => void;
  title: string;
}) {
  return (
    <div className="w-full bg-base-300 p-1 flex justify-between">
      <span className="capitalize">{title}</span>
      <div className="flex gap-4 items-center justify-center">
        {handleBack ? (
          <button
            tabIndex={1}
            role="button"
            className={`btn btn-active btn-xs`}
            onClick={handleBack}
          >
            <FaArrowLeft /> Back
          </button>
        ) : null}
        <button
          onClick={handleClose}
          className="grid place-items-center cursor-pointer mx-1"
        >
          <FaX />
        </button>
      </div>
    </div>
  );
}
