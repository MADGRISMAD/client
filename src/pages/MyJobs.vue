<template>
  <DefaultLayout>
    <div class="max-w-5xl mx-auto py-10 px-4">
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-bold">Mis vacantes publicadas</h1>
        <RouterLink to="/jobs/create" class="bg-emerald-600 text-white px-4 py-2 rounded hover:bg-emerald-700">
          Publicar nueva vacante
        </RouterLink>
      </div>

      <div v-if="jobs.length === 0" class="text-gray-500">Aún no has publicado vacantes.</div>

      <div v-else class="grid gap-4">
        <div v-for="job in jobs" :key="job._id" class="border p-4 rounded shadow-sm bg-white relative group">
          <h2 class="text-lg font-semibold">{{ job.title }}</h2>
          <p class="text-sm text-gray-600 mb-2">{{ job.description.slice(0, 100) }}...</p>
          <div class="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <span>{{ job.salaryRange.min }} - {{ job.salaryRange.max }} {{ job.salaryRange.currency }} / {{ job.salaryRange.type }}</span>
            <span v-if="job.isRemote">• Remoto</span>
            <span v-if="job.highlighted">• Destacado</span>
          </div>

          <div class="flex gap-3 mt-2">
            <RouterLink
              :to="`/jobs/${job._id}/applicants`"
              class="text-sm text-emerald-600 hover:underline"
            >
              Ver aplicantes
            </RouterLink>
            <button @click="openEditModal(job)" class="text-sm text-blue-600 hover:underline">Editar</button>
            <button @click="deleteJob(job._id)" class="text-sm text-red-600 hover:underline">Eliminar</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal de edición -->
    <div v-if="editingJob" class="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-md w-full max-w-lg relative">
        <h2 class="text-xl font-bold mb-4">Editar vacante</h2>
        <form @submit.prevent="handleUpdate">
          <input v-model="form.title" class="w-full border px-3 py-2 rounded mb-3" placeholder="Título" />
          <textarea v-model="form.description" rows="4" class="w-full border px-3 py-2 rounded mb-3" placeholder="Descripción" />
          <input v-model="tagsString" class="w-full border px-3 py-2 rounded mb-3" placeholder="Etiquetas (coma)" />
          <div class="flex justify-between gap-2 mb-3">
            <input v-model.number="form.salaryRange.min" type="number" class="w-full border px-3 py-2 rounded" placeholder="Min $" />
            <input v-model.number="form.salaryRange.max" type="number" class="w-full border px-3 py-2 rounded" placeholder="Max $" />
          </div>
          <div class="flex justify-end gap-3 mt-4">
            <button type="button" @click="editingJob = null" class="text-sm text-gray-600">Cancelar</button>
            <button type="submit" class="bg-emerald-600 text-white px-4 py-2 rounded">Guardar cambios</button>
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
