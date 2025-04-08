<template>
  <DefaultLayout>
    <div class="max-w-6xl mx-auto mt-10 px-4">
      <h1 class="text-2xl font-bold mb-6">Ofertas disponibles</h1>

      <!-- Buscador y Filtros -->
      <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
        <input
          v-model="search"
          type="text"
          placeholder="Buscar por título, descripción o tags..."
          class="flex-1 min-w-[200px] border px-4 py-2 rounded-md text-sm"
        />
        <div class="flex gap-2 text-sm">
          <button
            @click="filters.remote = !filters.remote"
            :class="['px-3 py-1 rounded-full border', filters.remote ? 'bg-green-100 text-green-700' : 'hover:bg-gray-50']"
          >
            📍 Remoto
          </button>
          <button
            @click="filters.freelance = !filters.freelance"
            :class="['px-3 py-1 rounded-full border', filters.freelance ? 'bg-green-100 text-green-700' : 'hover:bg-gray-50']"
          >
            💻 Freelance
          </button>
          <button
            @click="filters.project = !filters.project"
            :class="['px-3 py-1 rounded-full border', filters.project ? 'bg-green-100 text-green-700' : 'hover:bg-gray-50']"
          >
            📦 Proyecto
          </button>
        </div>
      </div>

      <div v-if="filteredJobs.length === 0" class="text-gray-500">
        No hay ofertas disponibles por ahora.
      </div>

      <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="job in filteredJobs"
          :key="job._id"
          class="border rounded-lg p-4 shadow-sm hover:shadow-md transition"
        >
          <div class="flex justify-between mb-2">
            <p class="text-xs text-blue-600" v-if="job.isRemote">Remoto</p>
            <p class="text-xs text-green-600 font-medium" v-if="job.highlighted">Destacado</p>
          </div>
          <h2 class="text-lg font-semibold text-green-700">{{ job.title }}</h2>
          <p class="text-sm text-gray-500 mb-2">{{ job.company || job.createdBy?.fullName }}</p>
          <p class="text-sm text-gray-600 mb-4">{{ job.description.slice(0, 100) }}...</p>

          <div class="flex flex-wrap gap-1 text-xs mb-3">
            <span
              v-for="tag in job.tags"
              :key="tag"
              class="bg-gray-100 text-gray-700 px-2 py-0.5 rounded-full"
            >
              {{ tag }}
            </span>
          </div>

          <p class="text-sm mb-3">
            <strong>${{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}</strong> /
            {{ job.salaryRange?.type }} · {{ job.duration || 'sin duración' }}
          </p>

          <router-link :to="`/jobs/${job._id}`" class="text-green-600 hover:underline text-sm">
            Ver más
          </router-link>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { onMounted, ref, computed } from 'vue'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'

const jobs = ref([])
const search = ref('')
const filters = ref({
  remote: false,
  freelance: false,
  project: false
})

onMounted(async () => {
  try {
    jobs.value = await JobService.getAll()
  } catch (err) {
    console.error('Error al cargar ofertas:', err)
  }
})

const filteredJobs = computed(() => {
  return jobs.value.filter(job => {
    const textMatch =
      job.title.toLowerCase().includes(search.value.toLowerCase()) ||
      job.description.toLowerCase().includes(search.value.toLowerCase()) ||
      job.tags?.some(tag => tag.toLowerCase().includes(search.value.toLowerCase()))

    const matchesRemote = !filters.value.remote || job.isRemote
    const matchesFreelance = !filters.value.freelance || job.salaryRange?.type === 'hora'
    const matchesProject = !filters.value.project || job.salaryRange?.type === 'proyecto'

    return textMatch && matchesRemote && matchesFreelance && matchesProject
  })
})
</script>
