// src/services/NotificationService.js
import axios from 'axios'

const API = 'http://localhost:5000/api/notifications'

export default {
  async getAll() {
    const token = localStorage.getItem('token')
    const res = await axios.get(API, {
      headers: { Authorization: `Bearer ${token}` }
    })
    return res.data
  },
  async markAsRead(id) {
    const token = localStorage.getItem('token')
    return axios.put(`${API}/${id}/read`, {}, {
      headers: { Authorization: `Bearer ${token}` }
    })
  },
  async markAll() {
    const token = localStorage.getItem('token')
    return axios.put(`${API}/mark-all`, {}, {
      headers: { Authorization: `Bearer ${token}` }
    })
  }
}
