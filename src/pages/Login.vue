<template>
    <DefaultLayout>
      <div class="max-w-md mx-auto mt-12 p-6 bg-white border shadow rounded-md">
        <h2 class="text-2xl font-semibold mb-4">Iniciar sesión</h2>
  
        <form @submit.prevent="handleLogin" class="space-y-4">
          <div>
            <label class="block mb-1 text-sm font-medium">Correo electrónico</label>
            <input
              v-model="email"
              type="email"
              class="w-full border rounded px-3 py-2"
              required
            />
          </div>
  
          <div>
            <label class="block mb-1 text-sm font-medium">Contraseña</label>
            <input
              v-model="password"
              type="password"
              class="w-full border rounded px-3 py-2"
              required
            />
          </div>
  
          <button
            type="submit"
            class="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700"
          >
            Entrar
          </button>
  
          <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
        </form>
      </div>
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
      router.push('/') // Redirige al home o a dashboard
    } catch (err) {
      error.value = err.response?.data?.message || 'Error al iniciar sesión'
    }
  }
  </script>
  