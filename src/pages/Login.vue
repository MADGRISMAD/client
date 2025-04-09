<template>
  <DefaultLayout>
    <section class="min-h-screen flex items-center justify-center bg-gradient-to-b from-white to-gray-50 py-16 px-4">
      <div class="w-full max-w-md bg-white border border-gray-200 rounded-2xl shadow-md p-8 space-y-6">
        <div class="text-center">
          <h2 class="text-3xl font-bold text-gray-900">Iniciar sesión</h2>
          <p class="text-sm text-gray-500 mt-1">Accede a tu cuenta para comenzar</p>
        </div>

        <form @submit.prevent="handleLogin" class="space-y-5">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Correo electrónico</label>
            <input
              v-model="email"
              type="email"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-2 focus:ring-emerald-500 focus:outline-none"
              placeholder="ejemplo@correo.com"
              required
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Contraseña</label>
            <input
              v-model="password"
              type="password"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-2 focus:ring-emerald-500 focus:outline-none"
              placeholder="••••••••"
              required
            />
          </div>

          <button
            type="submit"
            class="w-full bg-emerald-600 hover:bg-emerald-700 text-white font-semibold py-2 rounded-lg transition"
          >
            Iniciar sesión
          </button>

          <p v-if="error" class="text-sm text-red-600 mt-2 text-center">{{ error }}</p>
        </form>
      </div>
    </section>
  </DefaultLayout>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuth } from '../composables/useAuth'
import DefaultLayout from '../layouts/DefaultLayout.vue'

const router = useRouter()
const { login } = useAuth()

const email = ref('')
const password = ref('')
const error = ref('')

const handleLogin = async () => {
  error.value = ''
  try {
    await login(email.value, password.value)
    router.push('/')
  } catch (err) {
    error.value = err.response?.data?.message || 'Error al iniciar sesión'
  }
}
</script>
