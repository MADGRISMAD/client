import axios from 'axios'

const API = 'http://localhost:5000/api/jobs'

export default {
  async getAll() {
    const res = await axios.get(API)
    return res.data
  },
  async getById(id) {
    const res = await axios.get(`http://localhost:5000/api/jobs/${id}`)
    return res.data
  },
  
  
  
}
