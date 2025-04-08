<template>
  <DefaultLayout>
    <div class="max-w-8xl mx-auto mt-10 px-4">
      <h1 class="text-2xl font-bold mb-6">Ofertas disponibles</h1>

      <!-- Buscador y Filtros -->
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 p-4 rounded-md ">
  <input
    v-model="search"
    type="text"
    placeholder="🔍 Buscar oportunidades…"
    class="flex-1 min-w-[250px] border border-gray-300 px-4 py-2 rounded-md text-sm focus:ring focus:ring-green-200"
  />
  <div class="flex gap-2 flex-wrap text-sm">
    <button
      @click="filters.remote = !filters.remote"
      :class="['px-4 py-1.5 rounded-full border', filters.remote ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-100']"
    >
      📍 Remoto
    </button>
    <button
      @click="filters.freelance = !filters.freelance"
      :class="['px-4 py-1.5 rounded-full border', filters.freelance ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-100']"
    >
      💻 Freelance
    </button>
    <button
      @click="filters.project = !filters.project"
      :class="['px-4 py-1.5 rounded-full border', filters.project ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-100']"
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
          class="border border-gray-200 p-6 rounded-lg shadow-lg hover:shadow-xl transition transform hover:-translate-y-1 bg-white"
        >
          <div class="flex items-center justify-between mb-4">
            <div v-if="job.highlighted" class="text-green-600 text-xs font-semibold bg-green-100 px-3 py-1 rounded-full">
              🌟 Destacado
            </div>
            <div v-if="job.isRemote" class="text-xs text-blue-600 bg-blue-100 px-3 py-1 rounded-full">
              🌍 Remoto
            </div>
          </div>

          <h3 class="text-xl font-bold text-gray-800 mb-2">{{ job.title }}</h3>
          <p class="text-gray-500 text-sm mb-4">{{ job.company || job.createdBy?.fullName }}</p>
          <p class="text-sm text-gray-700 mb-4 line-clamp-3">{{ job.description }}</p>

          <div class="flex flex-wrap gap-2 mb-4">
            <span
              v-for="tag in job.tags"
              :key="tag"
              class="bg-gray-200 text-gray-700 px-3 py-1 text-xs rounded-full"
            >
              #{{ tag }}
            </span>
          </div>

          <p class="text-sm font-medium text-gray-800 mb-4">
            💰 {{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}/{{ job.salaryRange?.type }} · ⏳ {{ job.duration || 'Sin duración' }}
          </p>

          <router-link
            :to="`/jobs/${job._id}`"
            class="inline-block bg-gradient-to-r from-green-500 to-green-600 text-white text-sm px-5 py-2 rounded-md hover:from-green-600 hover:to-green-700 transition"
          >
            Ver detalles
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
