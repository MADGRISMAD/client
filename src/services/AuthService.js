import axios from 'axios'

const API = 'http://localhost:5000/api/users'

export default {
  async login(email, password) {
    const res = await axios.post(`${API}/login`, { email, password })
    const { token, user } = res.data
    localStorage.setItem('token', token)
    localStorage.setItem('user', JSON.stringify(user))
    return user
  },

  async register(data) {
    const res = await axios.post(`${API}/register`, data)
    return res.data
  },

  logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  },

  getToken() {
    return localStorage.getItem('token')
  },

  getUser() {
    return JSON.parse(localStorage.getItem('user'))
  },

  isLoggedIn() {
    return !!localStorage.getItem('token')
  }
}
