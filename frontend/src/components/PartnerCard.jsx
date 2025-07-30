function PartnerCard({ partner }) {
  return (
    <div className="bg-white border border-gray-200 shadow-lg rounded-xl p-5 hover:shadow-xl transition-shadow">
      <h2 className="text-xl font-semibold text-black mb-2">
        {partner.name}
      </h2>
        <p className="text-[#e60000] text-sm mb-1">
        <strong>Website:</strong>{" "}
        <a
          href={`https://${partner.website_url}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-[#e60000] hover:underline"
        >
          {partner.website_url}
        </a>
        </p>
        <p className="text-[#e60000] text-sm">
            <strong>Email:</strong>{" "}
            <a
              href={`mailto:${partner.email}`}
              className="text-[#e60000] hover:underline"
            >
            {partner.email}
        </a>
        </p>
    </div>
    );
}

export default PartnerCard;
