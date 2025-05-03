<template>
    <DefaultLayout>
      <div class="max-w-4xl mx-auto mt-6 px-4">
        <div class="flex items-center gap-4 mb-4">
          <router-link
            to="/my-jobs"
            class="inline-flex items-center gap-2 text-sm text-gray-600 hover:text-emerald-600 transition-colors"
          >
            <span>⬅️</span>
            Volver
          </router-link>
          <h1 class="text-xl font-bold text-gray-900">Publicar una nueva vacante</h1>
        </div>
  
        <form @submit.prevent="handleSubmit" class="space-y-4 bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
          <!-- Título -->
          <div class="space-y-1">
            <label class="block text-sm font-medium text-gray-700">Título del puesto</label>
            <input 
              v-model="form.title" 
              class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
              placeholder="Ej: Desarrollador Frontend Senior"
              required 
            />
          </div>
  
          <!-- Descripción -->
          <div class="space-y-1">
            <label class="block text-sm font-medium text-gray-700">Descripción</label>
            <textarea 
              v-model="form.description" 
              class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
              rows="4" 
              placeholder="Describe las responsabilidades, requisitos y beneficios del puesto..."
              required 
            />
          </div>
  
          <!-- Etiquetas -->
          <div class="space-y-1">
            <label class="block text-sm font-medium text-gray-700">Etiquetas (separadas por coma)</label>
            <input 
              v-model="tagsString" 
              class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
              placeholder="Ej: React, JavaScript, TypeScript"
            />
          </div>
  
          <!-- Remoto -->
          <div class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg">
            <input 
              type="checkbox" 
              v-model="form.isRemote" 
              id="remote" 
              class="w-4 h-4 text-emerald-600 border-gray-300 rounded focus:ring-emerald-500"
            />
            <label for="remote" class="text-sm font-medium text-gray-700">Trabajo remoto</label>
          </div>
  
          <!-- Salario -->
          <div class="space-y-2">
            <h3 class="text-sm font-medium text-gray-700">Rango salarial</h3>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
              <div class="space-y-1">
                <label class="block text-xs text-gray-500">Mínimo</label>
                <input 
                  v-model="form.salaryRange.min" 
                  type="number" 
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
                  placeholder="0"
                />
              </div>
              <div class="space-y-1">
                <label class="block text-xs text-gray-500">Máximo</label>
                <input 
                  v-model="form.salaryRange.max" 
                  type="number" 
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
                  placeholder="0"
                />
              </div>
              <div class="space-y-1">
                <label class="block text-xs text-gray-500">Tipo</label>
                <select 
                  v-model="form.salaryRange.type" 
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors"
                >
                  <option value="hora">Por hora</option>
                  <option value="mes">Por mes</option>
                  <option value="proyecto">Por proyecto</option>
                </select>
              </div>
              <div class="space-y-1">
                <label class="block text-xs text-gray-500">Moneda</label>
                <select 
                  v-model="form.salaryRange.currency" 
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors"
                >
                  <option value="USD">USD</option>
                  <option value="MXN">MXN</option>
                  <option value="EUR">EUR</option>
                </select>
              </div>
            </div>
          </div>
  
          <!-- Duración -->
          <div class="space-y-1">
            <label class="block text-sm font-medium text-gray-700">Duración del proyecto</label>
            <input 
              v-model="form.duration" 
              class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors" 
              placeholder="Ej: 3 meses"
            />
          </div>
  
          <!-- ¿Destacar vacante? -->
          <div class="flex items-center gap-2 p-2 bg-gray-50 rounded-lg">
            <input 
              type="checkbox" 
              v-model="form.highlighted" 
              id="highlighted" 
              class="w-4 h-4 text-emerald-600 border-gray-300 rounded focus:ring-emerald-500"
            />
            <label for="highlighted" class="text-sm font-medium text-gray-700">Destacar esta vacante</label>
          </div>
  
          <!-- Mensajes de estado -->
          <div v-if="success" class="p-3 bg-emerald-50 text-emerald-700 rounded-lg text-sm">
            {{ success }}
          </div>
          <div v-if="error" class="p-3 bg-red-50 text-red-700 rounded-lg text-sm">
            {{ error }}
          </div>
  
          <!-- Botón -->
          <button 
            class="w-full bg-emerald-600 text-white px-4 py-2 rounded-lg font-medium hover:bg-emerald-700 transition-colors focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2" 
            type="submit"
            :disabled="isSubmitting"
          >
            <span v-if="isSubmitting">Publicando...</span>
            <span v-else>Publicar vacante</span>
          </button>
        </form>
      </div>
    </DefaultLayout>
  </template>
  
  <script setup>
  import { ref, reactive, watch } from 'vue'
  import { useRouter } from 'vue-router'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  import axios from 'axios'
  
  const router = useRouter()
  const isSubmitting = ref(false)
  
  const form = reactive({
    title: '',
    description: '',
    tags: [],
    isRemote: false,
    salaryRange: {
      min: null,
      max: null,
      type: 'hora',
      currency: 'USD'
    },
    duration: '',
    highlighted: false
  })
  
  const tagsString = ref('')
  watch(tagsString, val => {
    form.tags = val.split(',').map(t => t.trim()).filter(Boolean)
  })
  
  const success = ref('')
  const error = ref('')
  
  const handleSubmit = async () => {
    try {
      isSubmitting.value = true
      error.value = ''
      success.value = ''
      
      const token = localStorage.getItem('token')
      await axios.post('http://localhost:5000/api/jobs', form, {
        headers: { Authorization: `Bearer ${token}` }
      })
      
      success.value = 'Vacante publicada con éxito. Redirigiendo...'
      setTimeout(() => router.push('/my-jobs'), 1500)
    } catch (err) {
      error.value = err.response?.data?.message || 'Error al publicar la vacante. Por favor, intenta nuevamente.'
    } finally {
      isSubmitting.value = false
    }
  }
  </script>
  