import { Link } from "react-router-dom";

function PageHeader({ isHome }) {
  return (
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-2xl font-semibold">Partner Portal</h1>
      <div className="space-x-2">
        <Link to="/">
          <button
            className={`px-4 py-2 rounded-xl shadow transition ${
              isHome
                ? "bg-gray-200 text-black hover:bg-gray-300"
                : "bg-red-600 text-white hover:bg-red-700"
            }`}
          >
            User View
          </button>
        </Link>
        <Link to="/admin">
          <button
            className={`px-4 py-2 rounded-xl shadow transition ${
              isHome
                ? "bg-red-600 text-white hover:bg-red-700"
                : "bg-gray-200 text-black hover:bg-gray-300"
            }`}
          >
            Admin Panel
          </button>
        </Link>
      </div>
    </div>
  );
}

export default PageHeader;
