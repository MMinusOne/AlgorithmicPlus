export default function Loading({
  isLoading,
  children,
}: {
  isLoading: boolean;
  children: React.ReactNode;
}) {
  return isLoading ? (
    <>
      <div className="w-full h-full grid place-items-center py-4">
        <span className="loading loading-spinner loading-xl"></span>
      </div>
    </>
  ) : (
    children
  );
}
