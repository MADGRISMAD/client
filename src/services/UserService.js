import { API_URL } from '../config'
import axios from 'axios'

const API = `${API_URL}/api/users`

export default {
  async updateProfile(data) {
    const token = localStorage.getItem('token')
    return await axios.put(`${API}/me`, data, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
  }
}
