import { ref } from 'vue'
import AuthService from '@/services/AuthService'

const user = ref(AuthService.getUser())

export function useAuth() {
  const login = async (email, password) => {
    const loggedInUser = await AuthService.login(email, password)
    user.value = loggedInUser
  }

  const logout = () => {
    AuthService.logout()
    user.value = null
  }

  return {
    user,
    login,
    logout,
    isLoggedIn: () => !!user.value,
    role: () => user.value?.role || null
  }
}
