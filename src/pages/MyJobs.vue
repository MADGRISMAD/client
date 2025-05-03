<template>
  <DefaultLayout>
    <div class="max-w-5xl mx-auto py-10 px-4">
      <div class="flex items-center justify-between mb-8">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Mis vacantes publicadas</h1>
          <p class="text-sm text-gray-500 mt-1">Administra tus vacantes publicadas</p>
        </div>
        <RouterLink 
          to="/jobs/create" 
          class="flex items-center gap-2 bg-emerald-600 text-white px-4 py-2 rounded-md hover:bg-emerald-700 transition-colors"
        >
          <span>+</span>
          <span>Publicar nueva vacante</span>
        </RouterLink>
      </div>

      <div v-if="jobs.length === 0" class="bg-white border border-gray-200 rounded-lg p-8 text-center">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-gray-100 flex items-center justify-center">
          <span class="text-2xl">📝</span>
        </div>
        <h3 class="text-lg font-medium text-gray-900 mb-2">No hay vacantes publicadas</h3>
        <p class="text-gray-500 mb-4">Comienza publicando tu primera vacante para encontrar candidatos</p>
        <RouterLink 
          to="/jobs/create" 
          class="inline-flex items-center gap-2 bg-emerald-600 text-white px-4 py-2 rounded-md hover:bg-emerald-700 transition-colors"
        >
          <span>+</span>
          <span>Publicar vacante</span>
        </RouterLink>
      </div>

      <div v-else class="grid gap-4">
        <div 
          v-for="job in jobs" 
          :key="job._id" 
          class="border border-gray-200 rounded-lg p-6 bg-white hover:shadow-md transition-shadow group"
        >
          <div class="flex items-start justify-between">
            <div class="flex-grow">
              <h2 class="text-lg font-semibold text-gray-900 mb-2">{{ job.title }}</h2>
              <p class="text-sm text-gray-600 mb-4 line-clamp-2">{{ job.description }}</p>
              
              <div class="flex flex-wrap gap-2 mb-4">
                <div class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded-full text-xs text-gray-600">
                  <span>💰</span>
                  <span>{{ job.salaryRange.min }} - {{ job.salaryRange.max }} {{ job.salaryRange.currency }} / {{ job.salaryRange.type }}</span>
                </div>
                <div v-if="job.isRemote" class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded-full text-xs text-gray-600">
                  <span>🌍</span>
                  <span>Remoto</span>
                </div>
                <div v-if="job.highlighted" class="flex items-center gap-1 px-2 py-1 bg-emerald-100 rounded-full text-xs text-emerald-700">
                  <span>⭐</span>
                  <span>Destacado</span>
                </div>
              </div>
            </div>
          </div>

          <div class="flex items-center justify-between pt-4 border-t border-gray-100">
            <div class="flex items-center gap-2">
              <RouterLink
                :to="`/jobs/${job._id}/applicants`"
                class="flex items-center gap-1 px-3 py-1.5 rounded-md text-sm text-emerald-600 hover:bg-emerald-50 transition-colors"
              >
                <span>👥</span>
                <span>Ver aplicantes</span>
              </RouterLink>
              <button 
                @click="openEditModal(job)" 
                class="flex items-center gap-1 px-3 py-1.5 rounded-md text-sm text-blue-600 hover:bg-blue-50 transition-colors"
              >
                <span>✏️</span>
                <span>Editar</span>
              </button>
              <button 
                @click="deleteJob(job._id)" 
                class="flex items-center gap-1 px-3 py-1.5 rounded-md text-sm text-red-600 hover:bg-red-50 transition-colors"
              >
                <span>🗑️</span>
                <span>Eliminar</span>
              </button>
            </div>
            <span class="text-xs text-gray-500">{{ formatDate(job.createdAt) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal de edición -->
    <div v-if="editingJob" class="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg w-full max-w-lg relative">
        <div class="p-6 border-b border-gray-100">
          <h2 class="text-xl font-bold text-gray-900">Editar vacante</h2>
          <p class="text-sm text-gray-500 mt-1">Actualiza los detalles de tu vacante</p>
        </div>
        
        <form @submit.prevent="handleUpdate" class="p-6">
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Título</label>
              <input 
                v-model="form.title" 
                class="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent" 
                placeholder="Título de la vacante" 
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Descripción</label>
              <textarea 
                v-model="form.description" 
                rows="4" 
                class="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent" 
                placeholder="Describe los detalles de la vacante"
              ></textarea>
            </div>
            
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Etiquetas</label>
              <input 
                v-model="tagsString" 
                class="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent" 
                placeholder="Etiquetas (separadas por comas)" 
              />
            </div>
            
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Salario mínimo</label>
                <input 
                  v-model.number="form.salaryRange.min" 
                  type="number" 
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent" 
                  placeholder="Min $" 
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Salario máximo</label>
                <input 
                  v-model.number="form.salaryRange.max" 
                  type="number" 
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent" 
                  placeholder="Max $" 
                />
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-3 mt-6">
            <button 
              type="button" 
              @click="editingJob = null" 
              class="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 rounded-md transition-colors"
            >
              Cancelar
            </button>
            <button 
              type="submit" 
              class="px-4 py-2 text-sm font-medium text-white bg-emerald-600 hover:bg-emerald-700 rounded-md transition-colors"
            >
              Guardar cambios
            </button>
          </div>
        </form>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, onMounted, reactive, watch } from 'vue'
import axios from 'axios'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import { RouterLink } from 'vue-router'

const jobs = ref([])
const editingJob = ref(null)
const form = reactive({
  title: '',
  description: '',
  tags: [],
  salaryRange: {
    min: null,
    max: null,
    type: 'hora',
    currency: 'USD'
  }
})
const tagsString = ref('')
const token = localStorage.getItem('token')

const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('es-ES', { 
    day: 'numeric', 
    month: 'short', 
    year: 'numeric' 
  })
}

const fetchJobs = async () => {
  const res = await axios.get('http://localhost:5000/api/jobs/my-jobs', {
    headers: { Authorization: `Bearer ${token}` }
  })
  jobs.value = res.data
}

onMounted(fetchJobs)

const openEditModal = (job) => {
  editingJob.value = job
  Object.assign(form, JSON.parse(JSON.stringify(job)))
  tagsString.value = job.tags.join(', ')
}

watch(tagsString, val => {
  form.tags = val.split(',').map(t => t.trim()).filter(Boolean)
})

const handleUpdate = async () => {
  try {
    await axios.put(`http://localhost:5000/api/jobs/${editingJob.value._id}`, form, {
      headers: { Authorization: `Bearer ${token}` }
    })
    editingJob.value = null
    fetchJobs()
  } catch (err) {
    alert('Error al actualizar la vacante')
  }
}

const deleteJob = async (id) => {
  if (confirm('¿Estás seguro de eliminar esta vacante?')) {
    try {
      await axios.delete(`http://localhost:5000/api/jobs/${id}`, {
        headers: { Authorization: `Bearer ${token}` }
      })
      fetchJobs()
    } catch (err) {
      alert('Error al eliminar la vacante')
    }
  }
}
</script>
