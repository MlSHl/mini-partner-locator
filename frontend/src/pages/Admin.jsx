import { useEffect, useState } from "react";
import { addCountryToPartner, createPartner, deletePartner, getAllPartnersDetailed, removeCountryFromPartner, updatePartner } from "../api/partner";
import PartnersTable from "../components/PartnersTable";
import PartnerFormModal from "../components/PartnerFormModal";
import { getAllCountries } from "../api/country";
import PageHeader from "../components/PageHeader";

function Admin() {
  const [partners, setPartners] = useState([]);
  const [selectedPartner, setSelectedPartner] = useState(null);
  const [showModal, setShowModal] = useState(false);
  const [allCountries, setAllCountries] = useState([]);

  useEffect(() => {
    fetchPartners();
    fetchCountries();
  }, []);

  const fetchPartners = async () => {
    const res = await getAllPartnersDetailed();
    setPartners(res.data);
  };

    const fetchCountries = async () => {
        const res = await getAllCountries();
        const countries = res.data;
        setAllCountries(countries);
    };

    const handleAdd = () => {
        setSelectedPartner(null);
        setShowModal(true);
    };

    const handleEdit = (partner) => {
        setSelectedPartner(partner);
        setShowModal(true); }; const handleDelete = async (id) => {
      try {
        await deletePartner(id);
        await fetchPartners();
      } catch (err) {
        console.error("Failed to delete:", err);
        alert("Could not delete partner.");
      }
    };

    const handleSubmit = async (partnerData) => {
      console.log("Submit partner:", partnerData);

      if (selectedPartner) {
        const partnerId = selectedPartner.id;
        
        const oldIds = selectedPartner.countries.map((c) => c.id);
        const newIds = partnerData.country_ids;

        const toAdd = newIds.filter((id) => !oldIds.includes(id));
        const toRemove = oldIds.filter((id) => !newIds.includes(id));
        console.log(toAdd);
        console.log(toRemove);

        await updatePartner(partnerId, {
          name: partnerData.name,
          email: partnerData.email,
          website_url: partnerData.website_url,
        });

        for (const countryId of toAdd) {
          await addCountryToPartner(partnerId, countryId)
        }

        for (const countryId of toRemove) {
          await removeCountryFromPartner(partnerId, countryId)
        }
      } else {
        await createPartner(partnerData);
      }

      await fetchPartners();
    };

      return ( 
        <div className="min-h-screen bg-gray-50 py-8 px-4 sm:px-8">
        <PageHeader isHome={false}/>
        <div className="p-8 max-w-7xl mx-auto">
          <h1 className="text-3xl font-bold mb-6">Admin Panel</h1>
          <button
            className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded mb-4"
            onClick={handleAdd}
          >
            Add Partner
          </button>
          <PartnersTable partners={partners} onEdit={handleEdit} />
          <PartnerFormModal
            isOpen={showModal}
            onClose={() => setShowModal(false)}
            existingPartner={selectedPartner}
            countries={allCountries}
            onSubmit={handleSubmit}
            onDelete={handleDelete}
          />
        </div>
          </div>
      );
    }

export default Admin;
