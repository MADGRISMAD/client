import axios from 'axios'

const API = 'http://localhost:5000/api/users'

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
