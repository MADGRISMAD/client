<template>
  <div class="min-h-[100dvh] flex flex-col bg-white text-gray-900">
    <!-- Barra superior: una sola línea, 64 px -->
    <header class="sticky top-0 z-40 w-full border-b border-gray-200 bg-white/90 backdrop-blur-md">
      <div class="mx-auto flex h-16 max-w-7xl items-center justify-between gap-6 px-4 sm:px-6">
        <router-link to="/" class="group flex items-center gap-2.5" aria-label="IAplica, inicio">
          <span class="grid h-8 w-8 place-items-center rounded-lg bg-emerald-600 text-white">
            <PhGraduationCap :size="20" weight="regular" />
          </span>
          <span class="text-lg font-semibold tracking-tight text-gray-900">IAplica</span>
        </router-link>

        <nav class="hidden items-center gap-1 md:flex" aria-label="Principal">
          <router-link to="/jobs" class="rounded-lg px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900">Explorar</router-link>
          <router-link to="/#categorias" class="rounded-lg px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900">Categorías</router-link>
          <router-link to="/#como-funciona" class="rounded-lg px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900">Cómo funciona</router-link>
          <router-link to="/empresas" class="rounded-lg px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900">Para empresas</router-link>
        </nav>

        <div class="hidden items-center gap-2 md:flex">
          <router-link
            v-if="isLoggedIn() && user?.role === 'employer'"
            to="/jobs/create"
            class="press rounded-lg bg-emerald-600 px-4 py-2 text-sm font-medium text-white hover:bg-emerald-700"
          >
            Publicar vacante
          </router-link>

          <!-- Notificaciones (clic o foco; también se cierra al hacer clic fuera) -->
          <div v-if="isLoggedIn()" class="relative" ref="notificationMenuRef">
            <button
              type="button"
              class="relative grid h-9 w-9 place-items-center rounded-lg text-gray-600 transition-colors hover:bg-gray-100 hover:text-gray-900"
              aria-label="Notificaciones"
              :aria-expanded="showNotif"
              @click="showNotif = !showNotif"
            >
              <PhBell :size="20" weight="regular" />
              <span v-if="unreadCount > 0" class="absolute -right-0.5 -top-0.5 grid h-4 min-w-4 place-items-center rounded-full bg-emerald-600 px-1 font-mono text-[10px] font-medium text-white">{{ unreadCount }}</span>
            </button>
            <div v-if="showNotif" class="absolute right-0 z-50 mt-2 max-h-96 w-96 overflow-auto rounded-xl border border-gray-200 bg-white shadow-lg">
              <div class="flex items-center justify-between border-b border-gray-200 px-4 py-3">
                <span class="text-sm font-semibold">Notificaciones</span>
                <button type="button" @click="markAll" class="text-xs font-medium text-emerald-700 hover:underline">Marcar todas como leídas</button>
              </div>
              <p v-if="notifications.length === 0" class="px-4 py-8 text-center text-sm text-gray-500">No hay notificaciones nuevas.</p>
              <button
                v-for="notif in notifications"
                :key="notif._id"
                type="button"
                class="block w-full border-b border-gray-100 px-4 py-3 text-left transition-colors last:border-0 hover:bg-gray-50"
                :class="{ 'bg-emerald-50': !notif.read }"
                @click="handleNotificationClick(notif)"
              >
                <p class="text-sm" :class="notif.read ? 'text-gray-600' : 'font-medium text-gray-900'">{{ notif.message }}</p>
                <span class="mt-1 block text-xs text-gray-500">{{ formatDate(notif.createdAt) }}</span>
              </button>
            </div>
          </div>

          <!-- Cuenta -->
          <div v-if="isLoggedIn()" class="relative" ref="profileMenuRef">
            <button
              type="button"
              class="flex items-center gap-2 rounded-lg border border-gray-200 py-1 pl-1 pr-2.5 transition-colors hover:bg-gray-100"
              :aria-expanded="showMenu"
              @click="showMenu = !showMenu"
            >
              <span class="grid h-7 w-7 place-items-center rounded-md bg-emerald-100 text-sm font-semibold text-emerald-700">{{ initial }}</span>
              <span class="text-sm font-medium">{{ firstName }}</span>
              <PhCaretDown :size="14" class="text-gray-500" />
            </button>
            <div v-if="showMenu" class="absolute right-0 z-50 mt-2 w-60 overflow-hidden rounded-xl border border-gray-200 bg-white shadow-lg">
              <div class="border-b border-gray-200 px-4 py-3">
                <p class="truncate text-sm font-semibold">{{ user?.fullName || 'Mi cuenta' }}</p>
                <p class="truncate text-xs text-gray-500">{{ user?.email }}</p>
              </div>
              <div class="p-1">
                <router-link :to="user?.role === 'employer' ? '/my-jobs' : '/profile'" class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-100" @click="showMenu = false">
                  <component :is="user?.role === 'employer' ? PhBriefcase : PhUser" :size="18" />
                  {{ user?.role === 'employer' ? 'Mis vacantes' : 'Mi perfil' }}
                </router-link>
                <router-link v-if="user?.role === 'student'" to="/my-applications" class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-100" @click="showMenu = false">
                  <PhNotePencil :size="18" />
                  Mis postulaciones
                </router-link>
                <button type="button" @click="handleLogout" class="flex w-full items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-100">
                  <PhSignOut :size="18" />
                  Cerrar sesión
                </button>
              </div>
            </div>
          </div>

          <template v-else>
            <router-link to="/login" class="rounded-lg px-3 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-100">Iniciar sesión</router-link>
            <router-link to="/register" class="press rounded-lg bg-emerald-600 px-4 py-2 text-sm font-medium text-white hover:bg-emerald-700">Registrarse</router-link>
          </template>
        </div>

        <button type="button" class="grid h-10 w-10 place-items-center rounded-lg text-gray-700 hover:bg-gray-100 md:hidden" :aria-expanded="showMobileMenu" aria-label="Menú" @click="showMobileMenu = !showMobileMenu">
          <PhX v-if="showMobileMenu" :size="22" />
          <PhList v-else :size="22" />
        </button>
      </div>

      <!-- Menú en celular -->
      <div v-if="showMobileMenu" class="absolute inset-x-0 top-16 border-b border-gray-200 bg-white shadow-lg md:hidden">
        <div class="space-y-1 px-4 py-3" @click="showMobileMenu = false">
          <router-link to="/jobs" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Explorar</router-link>
          <router-link to="/#categorias" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Categorías</router-link>
          <router-link to="/#como-funciona" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Cómo funciona</router-link>
          <router-link to="/empresas" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Para empresas</router-link>
          <template v-if="isLoggedIn()">
            <router-link v-if="user?.role === 'employer'" to="/jobs/create" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Publicar vacante</router-link>
            <router-link :to="user?.role === 'employer' ? '/my-jobs' : '/profile'" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">{{ user?.role === 'employer' ? 'Mis vacantes' : 'Mi perfil' }}</router-link>
            <router-link v-if="user?.role === 'student'" to="/my-applications" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Mis postulaciones</router-link>
            <button type="button" @click="handleLogout" class="block w-full rounded-lg px-3 py-2.5 text-left text-base font-medium text-gray-700 hover:bg-gray-100">Cerrar sesión</button>
          </template>
          <template v-else>
            <router-link to="/login" class="block rounded-lg px-3 py-2.5 text-base font-medium text-gray-700 hover:bg-gray-100">Iniciar sesión</router-link>
            <router-link to="/register" class="mt-2 block rounded-lg bg-emerald-600 px-3 py-2.5 text-center text-base font-medium text-white">Registrarse</router-link>
          </template>
        </div>
      </div>
    </header>

    <main class="flex-grow bg-gray-50">
      <slot />
    </main>

    <footer class="border-t border-gray-200 bg-white">
      <div class="mx-auto grid max-w-7xl gap-10 px-4 py-14 sm:px-6 md:grid-cols-[1.4fr_1fr_1fr_1fr]">
        <div>
          <router-link to="/" class="flex items-center gap-2.5" aria-label="IAplica, inicio">
            <span class="grid h-8 w-8 place-items-center rounded-lg bg-emerald-600 text-white"><PhGraduationCap :size="20" /></span>
            <span class="text-lg font-semibold tracking-tight">IAplica</span>
          </router-link>
          <p class="mt-4 max-w-xs text-sm leading-relaxed text-gray-600">Empleos y prácticas para estudiantes universitarios de Latinoamérica, con búsqueda por IA.</p>
        </div>
        <div>
          <h3 class="text-sm font-semibold">Estudiantes</h3>
          <ul class="mt-4 space-y-2.5 text-sm text-gray-600">
            <li><router-link to="/jobs" class="hover:text-gray-900">Explorar</router-link></li>
            <li><router-link to="/#categorias" class="hover:text-gray-900">Categorías</router-link></li>
            <li><router-link to="/#como-funciona" class="hover:text-gray-900">Cómo funciona</router-link></li>
          </ul>
        </div>
        <div>
          <h3 class="text-sm font-semibold">Empresas</h3>
          <ul class="mt-4 space-y-2.5 text-sm text-gray-600">
            <li><router-link to="/empresas" class="hover:text-gray-900">Para empresas</router-link></li>
            <li><router-link to="/jobs/create" class="hover:text-gray-900">Publicar vacante</router-link></li>
          </ul>
        </div>
        <div>
          <h3 class="text-sm font-semibold">Cuenta</h3>
          <ul class="mt-4 space-y-2.5 text-sm text-gray-600">
            <li><router-link to="/login" class="hover:text-gray-900">Iniciar sesión</router-link></li>
            <li><router-link to="/register" class="hover:text-gray-900">Registrarse</router-link></li>
          </ul>
        </div>
      </div>
      <div class="border-t border-gray-200">
        <p class="mx-auto max-w-7xl px-4 py-5 text-sm text-gray-500 sm:px-6">© 2026 IAplica. Todos los derechos reservados.</p>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { API_URL } from '../config'
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAuth } from '../composables/useAuth'
import { useRouter } from 'vue-router'
import axios from 'axios'
import { PhGraduationCap, PhBell, PhBriefcase, PhUser, PhNotePencil, PhSignOut, PhList, PhX, PhCaretDown } from '@phosphor-icons/vue'

const { logout, isLoggedIn, user } = useAuth()
const router = useRouter()

const showMenu = ref(false)
const showNotif = ref(false)
const showMobileMenu = ref(false)
const notifications = ref([])
const unreadCount = ref(0)

const firstName = computed(() => user.value?.fullName?.split(' ')[0] || 'Mi cuenta')
const initial = computed(() => (user.value?.fullName || 'U').charAt(0).toUpperCase())

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
    const res = await axios.get(`${API_URL}/api/notifications`, {
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
    await axios.put(`${API_URL}/api/notifications/${notif._id}/read`, null, {
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
    await axios.put(`${API_URL}/api/notifications/mark-all`, null, {
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
