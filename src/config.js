// Dirección de la API. En desarrollo apunta al servidor local; en producción se define VITE_API_URL al compilar.
export const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:4000'
