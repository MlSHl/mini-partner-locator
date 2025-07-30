import { useEffect, useState } from "react";

function PartnerFormModal({ isOpen, onClose, onSubmit, countries, existingPartner, onDelete }) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [websiteUrl, setWebsiteUrl] = useState("");
  const [selectedCountries, setSelectedCountries] = useState([]);

  useEffect(() => {
    if (existingPartner) {
        setName(existingPartner.name);
        setEmail(existingPartner.email);
        setWebsiteUrl(existingPartner.website_url);
        setSelectedCountries(existingPartner.countries.map((c) => c.id));
    } else {
      setName("");
      setEmail("");
      setWebsiteUrl("");
      setSelectedCountries([]);
    }
  }, [existingPartner]);

    const handleSubmit = (e) => {
        e.preventDefault();
        let website_url = websiteUrl;
        let result = {name, email, website_url, country_ids: selectedCountries};
        onSubmit(result);
    onClose();
  };


  if (!isOpen) return null;

  return (
<div className="fixed inset-0 bg-white/60 backdrop-blur-md flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-lg w-full max-w-xl p-6 relative">
        <h2 className="text-2xl font-bold mb-4">
          {existingPartner ? "Edit Partner" : "Add Partner"}
        </h2>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium">Name</label>
            <input
              type="text"
              className="mt-1 p-2 w-full border rounded"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium">Email</label>
            <input
              type="email"
              className="mt-1 p-2 w-full border rounded"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium">Website URL</label>
            <input
              type="text"
              className="mt-1 p-2 w-full border rounded"
              value={websiteUrl}
              onChange={(e) => setWebsiteUrl(e.target.value)}
              required
            />
          </div>

        <div>
          <label className="block text-sm font-medium mb-2">Countries</label>
          <div className="grid grid-cols-2 gap-2 max-h-40 overflow-y-auto border rounded p-2">
            {countries.map((c) => (
              <label key={c.id} className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  value={c.id}
                  checked={selectedCountries.includes(c.id)}
                  onChange={(e) => {
                    const id = c.id;
                    if (e.target.checked) {
                      setSelectedCountries((prev) => [...prev, id]);
                    } else {
                      setSelectedCountries((prev) => prev.filter((existingId) => existingId !== id));
                    }
                  }}
                />
                <span>{c.name}</span>
              </label>
            ))}
          </div>
        </div>


          <div className="flex justify-end space-x-4 mt-6">

            {existingPartner && (
              <button
                type="button"
                onClick={() => {
                  if (confirm("Are you sure you want to delete this partner?")) {
                    onDelete(existingPartner.id);
                    onClose();
                  }
                }}
                className="px-4 py-2 bg-red-100 hover:bg-red-200 text-red-700 rounded"
              >
                Delete Partner
              </button>
            )}
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-gray-300 hover:bg-gray-400 text-gray-800 rounded"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-[#e60000] hover:bg-red-700 text-white rounded"
            >
              {existingPartner ? "Save Changes" : "Add Partner"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default PartnerFormModal;
