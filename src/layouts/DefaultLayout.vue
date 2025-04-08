<template>
  <div class="min-h-screen flex flex-col">
    <!-- Navbar -->
    <nav class="bg-white border-b border-gray-200 px-6 py-3 flex items-center justify-between shadow-sm">
      <!-- Logo -->
      <router-link to="/" class="flex items-center gap-2 font-bold text-green-600 text-xl">
        <span class="text-2xl">🎓</span>
        internships.gg
      </router-link>

      <!-- Middle nav -->
      <div class="hidden md:flex gap-6 text-sm font-medium text-gray-600">
        <router-link to="/jobs" class="hover:text-green-600">Explorar</router-link>
        <a href="#" class="hover:text-green-600">Categorías</a>
        <a href="#" class="hover:text-green-600">Cómo funciona</a>
        <a href="#" class="hover:text-green-600">Para empresas</a>

        <router-link
          v-if="isLoggedIn() && user?.role === 'employer'"
          to="/jobs/create"
          class="text-green-600 font-semibold hover:underline"
        >
          Publicar vacante
        </router-link>
      </div>

      <!-- Right side -->
      <div class="flex items-center gap-3">
        <!-- Notificaciones -->
        <button class="relative p-2 rounded hover:bg-gray-100">
          <span class="text-xl">🔔</span>
          <span class="absolute top-1 right-1 w-2 h-2 bg-green-500 rounded-full"></span>
        </button>

        <template v-if="isLoggedIn()">
          <!-- Menú de usuario -->
          <div class="relative group">
            <button class="px-4 py-2 rounded-md text-sm bg-gray-50 hover:bg-gray-100 border border-gray-300">
              Perfil ▾
            </button>
            <div class="absolute right-0 mt-2 w-48 bg-white border rounded-md shadow-lg hidden group-hover:block z-10">
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
          </div>
        </template>

        <template v-else>
          <router-link
            to="/login"
            class="border border-gray-300 px-4 py-2 rounded-md text-sm hover:bg-gray-50"
          >
            Iniciar sesión
          </router-link>
          <router-link
            to="/register"
            class="bg-green-500 text-white px-4 py-2 rounded-md text-sm hover:bg-green-600"
          >
            Registrarse
          </router-link>
        </template>
      </div>
    </nav>

    <!-- Main content -->
    <main class="flex-grow p-4 bg-gray-50">
      <slot />
    </main>

    <!-- Footer -->
    <footer class="bg-white border-t border-gray-100 py-10 px-6 text-sm text-gray-600">
      <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8">
        <!-- Branding -->
        <div>
          <div class="flex items-center gap-2 font-bold text-green-600 text-xl mb-2">
            🎓 internships.gg
          </div>
          <p class="text-gray-500">
            Conectando talento universitario con oportunidades laborales desde 2023.
          </p>
        </div>

        <!-- Estudiantes -->
        <div>
          <h4 class="font-semibold text-gray-800 mb-2">Para estudiantes</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline">Cómo funciona</a></li>
            <li><a href="#" class="hover:underline">Crear perfil</a></li>
            <li><a href="#" class="hover:underline">Buscar oportunidades</a></li>
            <li><a href="#" class="hover:underline">Recursos</a></li>
          </ul>
        </div>

        <!-- Empresas -->
        <div>
          <h4 class="font-semibold text-gray-800 mb-2">Para empresas</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline">Publicar ofertas</a></li>
            <li><a href="#" class="hover:underline">Buscar talento</a></li>
            <li><a href="#" class="hover:underline">Planes y precios</a></li>
            <li><a href="#" class="hover:underline">Casos de éxito</a></li>
          </ul>
        </div>

        <!-- Contacto -->
        <div>
          <h4 class="font-semibold text-gray-800 mb-2">Contacto</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline">Soporte</a></li>
            <li><a href="#" class="hover:underline">Prensa</a></li>
            <li><a href="#" class="hover:underline">Términos y condiciones</a></li>
            <li><a href="#" class="hover:underline">Privacidad</a></li>
          </ul>
        </div>
      </div>

      <div class="text-center text-xs text-gray-400 mt-10 border-t pt-4">
        © {{ new Date().getFullYear() }} internships.gg. Todos los derechos reservados.
      </div>
    </footer>
  </div>
</template>

<script setup>
import { useAuth } from '../composables/useAuth'
import { useRouter } from 'vue-router'

const { logout, isLoggedIn, user } = useAuth()
const router = useRouter()

const handleLogout = () => {
  logout()
  router.push('/login')
}
</script>
