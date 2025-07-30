import api from './axios';

export const getPartnersByCountry = (countryName) => {  
    return api.get(`/partners/by-country/${countryName}`);
};

export const getAllPartnersDetailed= () => {
    return api.get(`/admin/partners`);
};

export const createPartner = (partner) => {
    return api.post("/admin/partners", partner);
};

export const updatePartner = (id, updatedPartnerData) => {
    return api.patch(`/admin/partners/${id}`, updatedPartnerData);
};

export const deletePartner = (id) => api.delete(`/admin/partners/${id}`);


export const addCountryToPartner= (partnerId, countryId) => {
    return api.patch(`/admin/partners/${partnerId}/country/${countryId}`);
};

export const removeCountryFromPartner= (partnerId, countryId) => {
      return api.delete(`/admin/partners/${partnerId}/country/${countryId}`);
};
