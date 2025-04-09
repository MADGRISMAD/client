<template>
  <div class="min-h-screen flex flex-col">
    <!-- Navbar actualizado -->
    <header class="sticky top-0 z-40 w-full  bg-white/95 backdrop-blur">
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
          <button class="relative">
            <span class="text-xl">🔔</span>
            <span class="absolute top-1 right-1 h-2 w-2 rounded-full bg-emerald-600"></span>
          </button>

          <template v-if="isLoggedIn()">
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

    <!-- Footer -->
    <footer class="bg-black border-t border-gray-800 py-10 px-6 text-sm text-gray-400">
      <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8">
        <!-- Branding -->
        <div>
          <div class="flex items-center gap-2 font-bold text-white text-xl mb-2">
            🎓 internships.gg
          </div>
          <p class="text-gray-500">
            Conectando talento universitario con oportunidades laborales desde 2023.
          </p>
        </div>

        <!-- Estudiantes -->
        <div>
          <h4 class="font-semibold text-white mb-2">Para estudiantes</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline text-gray-400">Cómo funciona</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Crear perfil</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Buscar oportunidades</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Recursos</a></li>
          </ul>
        </div>

        <!-- Empresas -->
        <div>
          <h4 class="font-semibold text-white mb-2">Para empresas</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline text-gray-400">Publicar ofertas</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Buscar talento</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Planes y precios</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Casos de éxito</a></li>
          </ul>
        </div>

        <!-- Contacto -->
        <div>
          <h4 class="font-semibold text-white mb-2">Contacto</h4>
          <ul class="space-y-1">
            <li><a href="#" class="hover:underline text-gray-400">Soporte</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Prensa</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Términos y condiciones</a></li>
            <li><a href="#" class="hover:underline text-gray-400">Privacidad</a></li>
          </ul>
        </div>
      </div>

      <div class="text-center text-xs text-gray-500 mt-10 border-t border-gray-800 pt-4">
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
