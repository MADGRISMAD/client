import axios from 'axios'
import { API_URL } from '../config'

const auth = () => ({ headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } })

export default {
  /** Busca por significado: «algo de diseño remoto para empezar». Devuelve vacantes con `matchScore` (0-100). */
  async search(query, { remote, limit } = {}) {
    const res = await axios.post(`${API_URL}/api/ai/search`, { query, remote, limit }, auth())
    return res.data.results
  },
  /** Vacantes que más encajan con el perfil del estudiante. */
  async recommended() {
    const res = await axios.get(`${API_URL}/api/ai/recommended`, auth())
    return res.data.results
  },
  /** Qué tanto encaja el estudiante con una vacante: { score, aiUsed, matchedSkills, missingSkills }. */
  async match(jobId) {
    const res = await axios.get(`${API_URL}/api/jobs/${jobId}/match`, auth())
    return res.data
  },
  /** Empleador: pule la descripción y sugiere etiquetas, requisitos, responsabilidades y beneficios. */
  async improveJob(title, description) {
    const res = await axios.post(`${API_URL}/api/ai/improve-job`, { title, description }, auth())
    return res.data
  },
  /** Estudiante: borrador de carta de presentación para una vacante. */
  async coverLetter(jobId) {
    const res = await axios.post(`${API_URL}/api/ai/cover-letter`, { jobId }, auth())
    return res.data.coverLetter
  }
}
