<template>
    <DefaultLayout>
      <div class="max-w-xl mx-auto mt-12 p-6 bg-white border shadow rounded-md">
        <h2 class="text-2xl font-semibold mb-4">Registrarse en internships.gg</h2>
  
        <form @submit.prevent="handleRegister" class="space-y-4">
          <!-- Rol -->
          <div>
            <label class="block text-sm font-medium mb-1">Tipo de cuenta</label>
            <select v-model="form.role" class="w-full border rounded px-3 py-2" required>
              <option value="student">Estudiante</option>
              <option value="employer">Empresa</option>
            </select>
          </div>
  
          <!-- Nombre -->
          <div>
            <label class="block text-sm font-medium mb-1">Nombre completo</label>
            <input v-model="form.fullName" type="text" class="w-full border rounded px-3 py-2" required />
          </div>
  
          <!-- Correo -->
          <div>
            <label class="block text-sm font-medium mb-1">Correo electrónico</label>
            <input v-model="form.email" type="email" class="w-full border rounded px-3 py-2" required />
          </div>
  
          <!-- Validación .edu -->
          <p v-if="form.role === 'student' && !emailEndsInEdu" class="text-red-500 text-sm">
            El correo debe terminar en .edu
          </p>
  
          <!-- Contraseña -->
          <div>
            <label class="block text-sm font-medium mb-1">Contraseña</label>
            <input v-model="form.password" type="password" class="w-full border rounded px-3 py-2" required />
          </div>
  
          <!-- Campos estudiante -->
          <div v-if="form.role === 'student'">
            <label class="block text-sm font-medium mb-1">Universidad</label>
            <input v-model="form.university" type="text" class="w-full border rounded px-3 py-2" />
            
            <label class="block mt-3 text-sm font-medium mb-1">Habilidades (separadas por coma)</label>
            <input v-model="skillsString" type="text" class="w-full border rounded px-3 py-2" />
          </div>
  
          <!-- Campos empresa -->
          <div v-if="form.role === 'employer'">
            <label class="block text-sm font-medium mb-1">Nombre de la empresa</label>
            <input v-model="form.company.name" type="text" class="w-full border rounded px-3 py-2" />
  
            <label class="block mt-3 text-sm font-medium mb-1">Sitio web</label>
            <input v-model="form.company.website" type="url" class="w-full border rounded px-3 py-2" />
  
            <label class="block mt-3 text-sm font-medium mb-1">Descripción</label>
            <textarea v-model="form.company.description" class="w-full border rounded px-3 py-2" rows="3" />
          </div>
  
          <!-- Botón -->
          <button
            type="submit"
            class="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700"
          >
            Registrarse
          </button>
  
          <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
          <p v-if="success" class="text-sm text-green-600 mt-2">{{ success }}</p>
        </form>
      </div>
    </DefaultLayout>
  </template>
  
  <script setup>
  import { ref, computed } from 'vue'
  import { useRouter } from 'vue-router'
  import AuthService from '../services/AuthService'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  
  const router = useRouter()
  const error = ref('')
  const success = ref('')
  const skillsString = ref('')
  
  const form = ref({
    role: 'student',
    fullName: '',
    email: '',
    password: '',
    university: '',
    skills: [],
    company: {
      name: '',
      website: '',
      description: ''
    }
  })
  
  const emailEndsInEdu = computed(() => form.value.email.trim().endsWith('.edu'))
  
  const handleRegister = async () => {
    error.value = ''
    success.value = ''
  
    if (form.value.role === 'student' && !emailEndsInEdu.value) {
      error.value = 'El correo debe terminar en .edu'
      return
    }
  
    if (form.value.role === 'student') {
      form.value.skills = skillsString.value
        .split(',')
        .map(s => s.trim())
        .filter(Boolean)
    }
  
    try {
      const res = await AuthService.register(form.value)
      success.value = res.message || 'Registro exitoso. Revisa tu correo si es necesario.'
      setTimeout(() => {
        router.push('/login')
      }, 1500)
    } catch (err) {
      error.value = err.response?.data?.message || 'Error al registrar'
    }
  }
  </script>
  