<template>
    <DefaultLayout>
      <div class="max-w-xl mx-auto mt-10">
        <h1 class="text-2xl font-bold mb-4">Mi perfil</h1>
  
        <form @submit.prevent="handleSave" class="space-y-4 text-gray-700">
          <!-- Común -->
          <div>
            <label class="block text-sm font-medium">Nombre completo</label>
            <input v-model="form.fullName" class="w-full border rounded px-3 py-2" />
          </div>
  
          <div>
            <label class="block text-sm font-medium">Correo electrónico</label>
            <input :value="user.email" disabled class="w-full border rounded px-3 py-2 bg-gray-100 text-gray-500" />
          </div>
  
          <!-- Estudiante -->
          <div v-if="user.role === 'student'">
            <div>
              <label class="block text-sm font-medium">Universidad</label>
              <input v-model="form.university" class="w-full border rounded px-3 py-2" />
            </div>
  
            <div>
              <label class="block text-sm font-medium">Habilidades (separadas por coma)</label>
              <input v-model="skillsString" class="w-full border rounded px-3 py-2" />
            </div>
          </div>
  
          <!-- Empresa -->
          <div v-else-if="user.role === 'employer'">
            <div>
              <label class="block text-sm font-medium">Nombre de la empresa</label>
              <input v-model="form.company.name" class="w-full border rounded px-3 py-2" />
            </div>
  
            <div>
              <label class="block text-sm font-medium">Sitio web</label>
              <input v-model="form.company.website" class="w-full border rounded px-3 py-2" />
            </div>
  
            <div>
              <label class="block text-sm font-medium">Descripción</label>
              <textarea v-model="form.company.description" class="w-full border rounded px-3 py-2" rows="3" />
            </div>
          </div>
  
          <!-- Botón -->
          <button
            type="submit"
            class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700"
          >
            Guardar cambios
          </button>
  
          <!-- Mensajes -->
          <p v-if="success" class="text-sm text-green-600">{{ success }}</p>
          <p v-if="error" class="text-sm text-red-500">{{ error }}</p>
        </form>
      </div>
    </DefaultLayout>
  </template>
  
  <script setup>
  import { ref, reactive, watch } from 'vue'
  import { useAuth } from '../composables/useAuth'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  import UserService from '../services/UserService'
  
  const { user } = useAuth()
  
  const form = reactive({
    fullName: user.value.fullName,
    university: user.value.university || '',
    skills: user.value.skills || [],
    company: {
      name: user.value.company?.name || '',
      website: user.value.company?.website || '',
      description: user.value.company?.description || ''
    }
  })
  
  const skillsString = ref(form.skills.join(', '))
  
  watch(skillsString, val => {
    form.skills = val.split(',').map(s => s.trim()).filter(Boolean)
  })
  
  const success = ref('')
  const error = ref('')
  
  const handleSave = async () => {
    error.value = ''
    success.value = ''
    try {
      await UserService.updateProfile(form)
      success.value = 'Perfil actualizado correctamente.'
    } catch (err) {
      error.value = err.response?.data?.message || 'Error al actualizar perfil.'
    }
  }
  </script>
  
  