import api from './axios';

export const getPartnersByCountry = (countryName) => {
  return api.get(`/partners/by-country/${countryName}`);
};
