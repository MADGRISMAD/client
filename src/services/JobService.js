import { API_URL } from '../config'
import axios from 'axios'

const API = `${API_URL}/api/jobs`

export default {
  /** Lista y busca. `params`: { q, remote, minSalary, limit, page }. */
  async getAll(params = {}) {
    const res = await axios.get(API, { params })
    return res.data
  },
  async getById(id) {
    const res = await axios.get(`${API_URL}/api/jobs/${id}`)
    return res.data
  },
  
  
  
}
