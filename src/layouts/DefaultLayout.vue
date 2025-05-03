<template>
  <div class="min-h-screen flex flex-col">
    <!-- Navbar -->
    <header class="sticky top-0 z-40 w-full bg-white/95 backdrop-blur border-b border-gray-100">
      <div class="max-w-7xl mx-auto flex h-16 items-center justify-between px-4 sm:px-6">
        <!-- Logo -->
        <router-link to="/" class="flex items-center gap-2 group">
          <div class="w-8 h-8 rounded-lg bg-emerald-100 flex items-center justify-center group-hover:bg-emerald-200 transition-colors">
            <span class="text-xl text-emerald-600">🎓</span>
          </div>
          <span class="text-xl font-bold text-emerald-600 group-hover:text-emerald-700 transition-colors">internships.gg</span>
        </router-link>

        <!-- Mobile menu button -->
        <button 
          @click="showMobileMenu = !showMobileMenu"
          class="md:hidden p-2 rounded-md hover:bg-gray-100 transition-colors"
        >
          <div class="w-6 h-5 flex flex-col justify-between">
            <span class="block w-full h-0.5 bg-gray-600 transition-transform" :class="{'rotate-45 translate-y-2': showMobileMenu}"></span>
            <span class="block w-full h-0.5 bg-gray-600 transition-opacity" :class="{'opacity-0': showMobileMenu}"></span>
            <span class="block w-full h-0.5 bg-gray-600 transition-transform" :class="{'-rotate-45 -translate-y-2': showMobileMenu}"></span>
          </div>
        </button>

        <!-- Desktop Links -->
        <div class="hidden md:flex items-center gap-1">
          <router-link 
            to="/jobs" 
            class="px-3 py-2 rounded-md text-sm font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Explorar
          </router-link>
          <a 
            href="#" 
            class="px-3 py-2 rounded-md text-sm font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Categorías
          </a>
          <a 
            href="#" 
            class="px-3 py-2 rounded-md text-sm font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Cómo funciona
          </a>
          <router-link 
            to="/empresas" 
            class="px-3 py-2 rounded-md text-sm font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Para empresas
          </router-link>
          <router-link
            v-if="isLoggedIn() && user?.role === 'employer'"
            to="/jobs/create"
            class="ml-2 px-4 py-2 rounded-md text-sm font-medium text-white bg-emerald-600 hover:bg-emerald-700 transition-colors"
          >
            Publicar vacante
          </router-link>
        </div>

        <!-- Right side -->
        <div class="hidden md:flex items-center gap-2">
          <!-- 🔔 Notificaciones -->
          <div class="relative" ref="notificationMenuRef">
            <div 
              class="relative p-2 rounded-full hover:bg-gray-100 transition-colors"
              @mouseenter="showNotif = true"
            >
              <span class="text-xl">🔔</span>
              <span
                v-if="unreadCount > 0"
                class="absolute -top-1 -right-1 h-4 w-4 rounded-full bg-emerald-600 text-white text-xs flex items-center justify-center"
              >
                {{ unreadCount }}
              </span>
            </div>
            <div
              v-if="showNotif"
              class="absolute right-0 mt-2 w-96 bg-white border border-gray-200 rounded-lg shadow-lg z-20 max-h-96 overflow-auto transition-all duration-200"
              @mouseenter="showNotif = true"
              @mouseleave="showNotif = false"
            >
              <div class="px-4 py-3 border-b border-gray-100 flex justify-between items-center bg-gray-50">
                <div class="flex items-center gap-2">
                  <span class="font-semibold text-sm text-gray-800">Notificaciones</span>
                  <span v-if="unreadCount > 0" class="px-2 py-0.5 bg-emerald-100 text-emerald-700 text-xs rounded-full">
                    {{ unreadCount }} sin leer
                  </span>
                </div>
                <button 
                  @click="markAll" 
                  class="text-xs text-emerald-600 hover:text-emerald-700 hover:underline transition-colors"
                >
                  Marcar todas como leídas
                </button>
              </div>
              <div v-if="notifications.length === 0" class="p-6 text-center">
                <span class="text-gray-400 text-sm">No hay notificaciones nuevas</span>
              </div>
              <div
                v-for="notif in notifications"
                :key="notif._id"
                class="px-4 py-3 border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
                :class="{ 
                  'bg-emerald-50/50 hover:bg-emerald-50': !notif.read,
                  'bg-white': notif.read
                }"
                @click="handleNotificationClick(notif)"
              >
                <div class="flex items-start gap-3">
                  <div class="flex-shrink-0 mt-1">
                    <div 
                      class="w-2 h-2 rounded-full"
                      :class="notif.read ? 'bg-gray-300' : 'bg-emerald-500'"
                    ></div>
                  </div>
                  <div class="flex-grow">
                    <p class="text-sm text-gray-800 mb-1">{{ notif.message }}</p>
                    <span class="text-xs text-gray-500">{{ formatDate(notif.createdAt) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Menú de perfil -->
          <template v-if="isLoggedIn()">
            <div class="relative" ref="profileMenuRef">
              <div 
                class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-gray-50 hover:bg-gray-100 border border-gray-200 transition-all duration-200 cursor-pointer group"
                @mouseenter="showMenu = true"
              >
                <div class="w-8 h-8 rounded-full bg-emerald-100 flex items-center justify-center">
                  <span class="text-emerald-600 text-sm font-medium">
                    {{ user?.name?.charAt(0) || 'U' }}
                  </span>
                </div>
                <div class="flex items-center gap-1">
                  <span class="text-sm font-medium text-gray-700">{{ user?.name?.split(' ')[0] || 'Usuario' }}</span>
                  <span class="text-gray-400 group-hover:text-gray-500 transition-colors">▾</span>
                </div>
              </div>
              <div
                v-if="showMenu"
                class="absolute right-0 mt-2 w-56 bg-white border border-gray-200 rounded-lg shadow-lg z-10 transition-all duration-200"
                @mouseenter="showMenu = true"
                @mouseleave="showMenu = false"
              >
                <div class="py-1">
                  <div class="px-4 py-2 border-b border-gray-100">
                    <div class="flex items-center gap-3">
                      <div class="w-10 h-10 rounded-full bg-emerald-100 flex items-center justify-center">
                        <span class="text-emerald-600 text-base font-medium">
                          {{ user?.name?.charAt(0) || 'U' }}
                        </span>
                      </div>
                      <div>
                        <p class="text-sm font-medium text-gray-900">{{ user?.name || 'Usuario' }}</p>
                        <p class="text-xs text-gray-500">{{ user?.email }}</p>
                      </div>
                    </div>
                  </div>
                  <router-link
                    :to="user?.role === 'employer' ? '/my-jobs' : '/profile'"
                    class="flex items-center gap-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 transition-colors"
                  >
                    <span class="text-gray-500">👤</span>
                    {{ user?.role === 'employer' ? 'Mis vacantes' : 'Mi perfil' }}
                  </router-link>
                  <router-link
                    v-if="user?.role === 'student'"
                    to="/my-applications"
                    class="flex items-center gap-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 transition-colors"
                  >
                    <span class="text-gray-500">📝</span>
                    Mis postulaciones
                  </router-link>
                  <div class="border-t border-gray-100 my-1"></div>
                  <button
                    @click="handleLogout"
                    class="w-full flex items-center gap-2 px-4 py-2 text-sm text-red-600 hover:bg-red-50 transition-colors"
                  >
                    <span class="text-gray-500">🚪</span>
                    Cerrar sesión
                  </button>
                </div>
              </div>
            </div>
          </template>

          <template v-else>
            <router-link 
              to="/login" 
              class="px-4 py-2 rounded-md text-sm font-medium text-gray-700 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
            >
              Iniciar sesión
            </router-link>
            <router-link
              to="/register"
              class="px-4 py-2 rounded-md text-sm font-medium text-white bg-emerald-600 hover:bg-emerald-700 transition-colors"
            >
              Registrarse
            </router-link>
          </template>
        </div>
      </div>

      <!-- Mobile menu -->
      <div 
        v-if="showMobileMenu"
        class="md:hidden absolute top-16 inset-x-0 bg-white border-b border-gray-100 shadow-lg"
      >
        <div class="px-4 py-3 space-y-1">
          <router-link 
            to="/jobs" 
            class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Explorar
          </router-link>
          <a 
            href="#" 
            class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Categorías
          </a>
          <a 
            href="#" 
            class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Cómo funciona
          </a>
          <router-link 
            to="/empresas" 
            class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
          >
            Para empresas
          </router-link>
          <template v-if="isLoggedIn()">
            <router-link
              :to="user?.role === 'employer' ? '/my-jobs' : '/profile'"
              class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
            >
              {{ user?.role === 'employer' ? 'Mis vacantes' : 'Mi perfil' }}
            </router-link>
            <router-link
              v-if="user?.role === 'student'"
              to="/my-applications"
              class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
            >
              Mis postulaciones
            </router-link>
            <button
              @click="handleLogout"
              class="block w-full text-left px-3 py-2 rounded-md text-base font-medium text-red-600 hover:bg-red-50 transition-colors"
            >
              Cerrar sesión
            </button>
          </template>
          <template v-else>
            <router-link 
              to="/login" 
              class="block px-3 py-2 rounded-md text-base font-medium text-gray-600 hover:text-emerald-600 hover:bg-gray-50 transition-colors"
            >
              Iniciar sesión
            </router-link>
            <router-link
              to="/register"
              class="block px-3 py-2 rounded-md text-base font-medium text-white bg-emerald-600 hover:bg-emerald-700 transition-colors"
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

    <!-- Footer -->
    <footer class="bg-black text-gray-300">
      <!-- Upper footer -->
      <div class="max-w-7xl mx-auto px-4 sm:px-6 py-12">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Sobre nosotros</h3>
            <ul class="space-y-2">
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Nuestra misión</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Equipo</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Carreras</a></li>
            </ul>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Para estudiantes</h3>
            <ul class="space-y-2">
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Explorar trabajos</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Categorías</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Cómo funciona</a></li>
            </ul>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Para empresas</h3>
            <ul class="space-y-2">
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Publicar vacante</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Planes</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Recursos</a></li>
            </ul>
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Legal</h3>
            <ul class="space-y-2">
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Términos</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Privacidad</a></li>
              <li><a href="#" class="hover:text-emerald-400 transition-colors">Cookies</a></li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Lower footer -->
      <div class="border-t border-gray-800 py-6">
        <div class="max-w-7xl mx-auto px-4 sm:px-6">
          <div class="flex flex-col md:flex-row justify-between items-center gap-4">
            <div class="flex items-center gap-2">
              <div class="w-8 h-8 rounded-lg bg-emerald-100 flex items-center justify-center">
                <span class="text-xl text-emerald-600">🎓</span>
              </div>
              <span class="text-lg font-bold text-white">internships.gg</span>
            </div>
            <div class="flex items-center gap-4">
              <a href="#" class="text-gray-400 hover:text-emerald-400 transition-colors">
                <span class="text-xl">📱</span>
              </a>
              <a href="#" class="text-gray-400 hover:text-emerald-400 transition-colors">
                <span class="text-xl">📧</span>
              </a>
              <a href="#" class="text-gray-400 hover:text-emerald-400 transition-colors">
                <span class="text-xl">💼</span>
              </a>
            </div>
            <p class="text-sm text-gray-400">
              © 2024 internships.gg. Todos los derechos reservados.
            </p>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useAuth } from '../composables/useAuth'
import { useRouter } from 'vue-router'
import axios from 'axios'

const { logout, isLoggedIn, user } = useAuth()
const router = useRouter()

const showMenu = ref(false)
const showNotif = ref(false)
const showMobileMenu = ref(false)
const notifications = ref([])
const unreadCount = ref(0)

// Referencias para los elementos del menú
const profileMenuRef = ref(null)
const notificationMenuRef = ref(null)

const handleClickOutside = (event) => {
  if (profileMenuRef.value && !profileMenuRef.value.contains(event.target)) {
    showMenu.value = false
  }
  if (notificationMenuRef.value && !notificationMenuRef.value.contains(event.target)) {
    showNotif.value = false
  }
}

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

const formatDate = (dateString) => {
  const date = new Date(dateString)
  const now = new Date()
  const diff = now - date
  
  if (diff < 60000) return 'Hace unos segundos'
  if (diff < 3600000) return `Hace ${Math.floor(diff/60000)} minutos`
  if (diff < 86400000) return `Hace ${Math.floor(diff/3600000)} horas`
  return date.toLocaleDateString('es-ES', { day: 'numeric', month: 'short' })
}

onMounted(() => {
  if (isLoggedIn()) fetchNotifications()
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
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
