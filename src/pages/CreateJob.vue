<template>
    <DefaultLayout>
      <div class="max-w-3xl mx-auto mt-10 px-4">
        <h1 class="text-2xl font-bold mb-6">Publicar una vacante</h1>
  
        <form @submit.prevent="handleSubmit" class="space-y-6 bg-white border rounded-md p-6 shadow-sm">
          <!-- Título -->
          <div>
            <label class="block text-sm font-medium mb-1">Título del puesto</label>
            <input v-model="form.title" class="w-full border rounded px-3 py-2" required />
          </div>
  
          <!-- Descripción -->
          <div>
            <label class="block text-sm font-medium mb-1">Descripción</label>
            <textarea v-model="form.description" class="w-full border rounded px-3 py-2" rows="6" required />
          </div>
  
          <!-- Etiquetas -->
          <div>
            <label class="block text-sm font-medium mb-1">Etiquetas (separadas por coma)</label>
            <input v-model="tagsString" class="w-full border rounded px-3 py-2" />
          </div>
  
          <!-- Remoto -->
          <div class="flex items-center gap-2">
            <input type="checkbox" v-model="form.isRemote" id="remote" />
            <label for="remote" class="text-sm">Trabajo remoto</label>
          </div>
  
          <!-- Salario -->
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <label class="block text-sm font-medium mb-1">Salario mínimo</label>
              <input v-model="form.salaryRange.min" type="number" class="w-full border rounded px-3 py-2" />
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Salario máximo</label>
              <input v-model="form.salaryRange.max" type="number" class="w-full border rounded px-3 py-2" />
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Tipo</label>
              <select v-model="form.salaryRange.type" class="w-full border rounded px-3 py-2">
                <option value="hora">Por hora</option>
                <option value="mes">Por mes</option>
                <option value="proyecto">Por proyecto</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Moneda</label>
              <select v-model="form.salaryRange.currency" class="w-full border rounded px-3 py-2">
                <option value="USD">USD</option>
                <option value="MXN">MXN</option>
                <option value="EUR">EUR</option>
              </select>
            </div>
          </div>
  
          <!-- Duración -->
          <div>
            <label class="block text-sm font-medium mb-1">Duración del proyecto</label>
            <input v-model="form.duration" class="w-full border rounded px-3 py-2" placeholder="Ej: 3 meses" />
          </div>
  
          <!-- ¿Destacar vacante? -->
          <div class="flex items-center gap-2">
            <input type="checkbox" v-model="form.highlighted" id="highlighted" />
            <label for="highlighted" class="text-sm">Destacar esta vacante</label>
          </div>
  
          <!-- Botón -->
          <button class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700" type="submit">
            Publicar vacante
          </button>
  
          <p v-if="success" class="text-green-600 mt-2 text-sm">{{ success }}</p>
          <p v-if="error" class="text-red-500 mt-2 text-sm">{{ error }}</p>
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
      const token = localStorage.getItem('token')
      await axios.post('http://localhost:5000/api/jobs', form, {
        headers: { Authorization: `Bearer ${token}` }
      })
      success.value = 'Vacante publicada con éxito.'
      setTimeout(() => router.push('/jobs'), 1500)
    } catch (err) {
      error.value = err.response?.data?.message || 'Error al publicar la vacante.'
    }
  }
  </script>
  