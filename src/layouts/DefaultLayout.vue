<template>
  <div class="min-h-screen flex flex-col">
    <!-- Navbar -->
    <header class="sticky top-0 z-40 w-full bg-white/95 backdrop-blur">
      <div class="max-w-7xl mx-auto flex h-16 items-center justify-between px-6">
        <!-- Logo -->
        <router-link to="/" class="flex items-center gap-2">
          <span class="text-xl">🎓</span>
          <span class="text-xl font-bold text-emerald-600">internships.gg</span>
        </router-link>

        <!-- Links -->
        <div class="hidden md:flex items-center gap-6 text-sm font-medium text-gray-700">
          <router-link to="/jobs" class="hover:text-emerald-600">Explorar</router-link>
          <a href="#" class="hover:text-emerald-600">Categorías</a>
          <a href="#" class="hover:text-emerald-600">Cómo funciona</a>
          <router-link to="/empresas" class="hover:text-emerald-600">Para empresas</router-link>
          <router-link
            v-if="isLoggedIn() && user?.role === 'employer'"
            to="/jobs/create"
            class="text-emerald-600 font-semibold hover:underline"
          >
            Publicar vacante
          </router-link>
        </div>

        <!-- Right side -->
        <div class="flex items-center gap-4">
          <!-- 🔔 Notificaciones -->
          <div class="relative" @mouseenter="showNotif = true" @mouseleave="showNotif = false">
            <button class="relative">
              <span class="text-xl">🔔</span>
              <span
                v-if="unreadCount > 0"
                class="absolute top-0 right-0 h-2 w-2 rounded-full bg-emerald-600"
              ></span>
            </button>
            <div
              v-if="showNotif"
              class="absolute right-0 mt-2 w-80 bg-white border rounded-md shadow-lg z-20 max-h-80 overflow-auto"
            >
              <div class="px-4 py-2 border-b flex justify-between items-center">
                <span class="font-semibold text-sm">Notificaciones</span>
                <button @click="markAll" class="text-xs text-emerald-600 hover:underline">
                  Marcar todas como leídas
                </button>
              </div>
              <div v-if="notifications.length === 0" class="p-4 text-sm text-gray-500">Sin notificaciones</div>
              <div
                v-for="notif in notifications"
                :key="notif._id"
                class="px-4 py-3 border-b hover:bg-gray-50 text-sm cursor-pointer"
                :class="{ 'bg-gray-100': !notif.read }"
                @click="handleNotificationClick(notif)"
              >
                {{ notif.message }}
              </div>
            </div>
          </div>

          <!-- Menú de perfil -->
          <template v-if="isLoggedIn()">
            <div class="relative" @mouseenter="showMenu = true" @mouseleave="showMenu = false">
              <button class="px-4 py-2 rounded-md text-sm bg-gray-50 hover:bg-gray-100 border border-gray-300">
                Perfil ▾
              </button>
              <transition name="fade">
                <div
                  v-if="showMenu"
                  class="absolute right-0 mt-2 w-48 bg-white border rounded-md shadow-lg z-10"
                >
                  <router-link
                    :to="user?.role === 'employer' ? '/my-jobs' : '/profile'"
                    class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  >
                    {{ user?.role === 'employer' ? 'Mis vacantes' : 'Mi perfil' }}
                  </router-link>
                  <router-link
                    v-if="user?.role === 'student'"
                    to="/my-applications"
                    class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  >
                    Mis postulaciones
                  </router-link>
                  <button
                    @click="handleLogout"
                    class="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50"
                  >
                    Cerrar sesión
                  </button>
                </div>
              </transition>
            </div>
          </template>

          <template v-else>
            <router-link to="/login" class="border border-gray-300 px-4 py-2 rounded-md text-sm hover:bg-gray-50">
              Iniciar sesión
            </router-link>
            <router-link
              to="/register"
              class="bg-emerald-600 text-white px-4 py-2 rounded-md text-sm hover:bg-emerald-700"
            >
              Registrarse
            </router-link>
          </template>
        </div>
      </div>
    </header>

    <!-- Main content -->
    <main class="flex-grow bg-gray-50">
      <slot />
    </main>

    <!-- Footer omitido por brevedad -->
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useAuth } from '../composables/useAuth'
import { useRouter } from 'vue-router'
import axios from 'axios'

const { logout, isLoggedIn, user } = useAuth()
const router = useRouter()

const showMenu = ref(false)
const showNotif = ref(false)
const notifications = ref([])
const unreadCount = ref(0)

const fetchNotifications = async () => {
  try {
    const token = localStorage.getItem('token')
    const res = await axios.get('http://localhost:5000/api/notifications', {
      headers: { Authorization: `Bearer ${token}` }
    })
    notifications.value = res.data
    unreadCount.value = res.data.filter(n => !n.read).length
  } catch (err) {
    console.error('Error cargando notificaciones', err)
  }
}

const handleNotificationClick = async notif => {
  if (!notif.read) {
    const token = localStorage.getItem('token')
    await axios.put(`http://localhost:5000/api/notifications/${notif._id}/read`, null, {
      headers: { Authorization: `Bearer ${token}` }
    })
    notif.read = true
    unreadCount.value--
  }
  if (notif.link) router.push(notif.link)
}

const markAll = async () => {
  try {
    const token = localStorage.getItem('token')
    await axios.put('http://localhost:5000/api/notifications/mark-all', null, {
      headers: { Authorization: `Bearer ${token}` }
    })
    notifications.value.forEach(n => (n.read = true))
    unreadCount.value = 0
  } catch (err) {
    console.error('Error al marcar todas como leídas', err)
  }
}

const handleLogout = () => {
  logout()
  router.push('/login')
}

onMounted(() => {
  if (isLoggedIn()) fetchNotifications()
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
