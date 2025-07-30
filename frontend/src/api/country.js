import api from './axios';

export const getRegions = () => api.get('/regions');
export const getCountriesByRegion = (region) => api.get(`/countries/${region}`);
export const getAllCountries = () => api.get(`/countries`);
