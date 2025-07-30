import { useEffect, useState } from "react";
import { getRegions, getCountriesByRegion } from "../api/country";
import { ChevronDownIcon, ChevronUpIcon } from "@heroicons/react/20/solid";
import { Link } from "react-router-dom";
function Home() {

    const [regions, setRegions] = useState([]);
    const [countriesByRegion, setCountriesByRegion] = useState({});
    const [expandedRegions, setExpandedRegions] = useState({});

    const toggleExpand = (region) => {
    setExpandedRegions((prev) => ({
      ...prev,
      [region]: !prev[region],
    }));
    };
    useEffect(() => {
        getRegions().then((response) => {
            setRegions(response.data);
            response.data.forEach((region) => {
                getCountriesByRegion(region).then((res) => {
                    setCountriesByRegion((prev) => ({
                        ...prev,
                        [region]: res.data,
                    }));
                });
            });
        }).catch((error) => {
            console.log("Failed to fetch regions, ", error);
        });
    }, []);

    return (
    <div className="min-h-screen bg-gray-50 py-10 px-4 sm:px-8">
        <h1 className="text-4xl font-bold text-center text-gray-800 mb-12 tracking-tight">
         Partners By Regions 
    </h1>

    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        {regions.map((region) => {
          const countries = countriesByRegion[region] || [];
          const isExpanded = expandedRegions[region];
          const visibleCountries = isExpanded ? countries : countries.slice(0, 3);
          const showToggle = countries.length > 3;

          return (
            <div
              key={region}
              className="bg-white/80 backdrop-blur-sm border border-gray-200 rounded-2xl shadow-lg p-6 hover:shadow-xl transition-all duration-200 flex flex-col"
            >
              <h2 className="text-lg font-semibold text-[#e60000] uppercase tracking-wider mb-3 border-b pb-2 border-[#e60000]">
                {region}
              </h2>

              <ul className="space-y-2 text-gray-700 text-sm mb-4 flex-1">
                {visibleCountries.map((country) => (
                <li key={country.name} className="hover:text-[#e60000] hover:underline cursor-pointer transition-colors">
                  <Link to={`/country/${country.name}`}>{country.name}</Link>
                </li>
                ))}
              </ul>

              {showToggle && (
                <button
                  onClick={() => toggleExpand(region)}
                  className="flex items-center gap-1 text-sm text-[#e60000] hover:underline font-medium">
                  {isExpanded ? "Show less" : `Show ${countries.length - 3} more`}
                  {isExpanded ? (<ChevronUpIcon className="w-4 h-4" />) 
                  : (<ChevronDownIcon className="w-4 h-4" />)}
                </button>
              )}
            </div>
          );
        })}
      </div>
    </div>
    );
}
export default Home;
