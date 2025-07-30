import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { getPartnersByCountry } from "../api/partner"
import PartnerCard from "../components/PartnerCard";

function CountryPage () {
    const { name } = useParams();
    const [partners, setPartners] = useState([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        getPartnersByCountry(name)
        .then((res) => setPartners(res.data))
        .catch((err) => console.error("Failed to fetch partners:", err))
        .finally(() => setLoading(false));
    }, [name]);

    return (
        <div className="max-w-7xl mx-auto p-8">
            <h1 className="text-3xl font-bold text-gray-800 mb-6">
                Partners in {name}
            </h1>

            {loading ? (<p>Loading partners...</p>
            ) : partners.length === 0 ? (
                <p className="text-gray-600">No partners found in this country.</p>
            ) : (
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {partners.map((partner, index) => (
                    <PartnerCard key={index} partner={partner} />
                ))}
            </div>
            )}
        </div>
    );
}

export default CountryPage;
