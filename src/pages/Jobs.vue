<template>
  <DefaultLayout>
    <div class="max-w-7xl mx-auto mt-10 px-4 p-6">
      <h1 class="text-3xl font-bold mb-6 text-gray-800">Ofertas disponibles</h1>

      <!-- Buscador y Filtros -->
      <div class="bg-white p-4 rounded-xl shadow-sm border border-gray-100 mb-6 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <input
          v-model="search"
          type="text"
          placeholder="🔍 Buscar oportunidades…"
          class="flex-1 min-w-[250px] border border-gray-200 px-4 py-2 rounded-md text-sm focus:ring focus:ring-green-200"
        />
        <div class="flex gap-2 flex-wrap text-sm">
          <button
            @click="filters.remote = !filters.remote"
            :class="['px-4 py-1.5 rounded-full border transition', filters.remote ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
          >📍 Remoto</button>
          <button
            @click="filters.freelance = !filters.freelance"
            :class="['px-4 py-1.5 rounded-full border transition', filters.freelance ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
          >💻 Freelance</button>
          <button
            @click="filters.project = !filters.project"
            :class="['px-4 py-1.5 rounded-full border transition', filters.project ? 'bg-green-100 text-green-700 border-green-400' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
          >📦 Proyecto</button>
        </div>
      </div>

      <div v-if="filteredJobs.length === 0" class="text-gray-500 text-center mt-10">
        No hay ofertas disponibles por ahora.
      </div>

      <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="job in filteredJobs"
          :key="job._id"
          :class="['bg-white rounded-xl shadow-sm border transition p-6 hover:shadow-md', job.highlighted ? 'md:col-span-2 border-emerald-100 bg-gradient-to-br from-white to-emerald-50' : 'border-gray-100']"
        >
          <div class="flex justify-between items-start mb-4">
            <div class="flex items-center gap-4">
              <div class="h-12 w-12 bg-gray-100 rounded-lg flex items-center justify-center text-lg font-bold text-emerald-600">
                {{ job.company?.[0] || job.createdBy?.fullName?.[0] || '?' }}
              </div>
              <div>
                <div v-if="job.highlighted" class="text-xs font-semibold text-emerald-800 bg-emerald-100 rounded-full px-3 py-1 mb-1 inline-block">
                  🌟 Destacado
                </div>
                <h3 class="text-lg font-bold text-gray-800">{{ job.title }}</h3>
                <p class="text-sm text-gray-500">{{ job.company || job.createdBy?.fullName }}</p>
              </div>
            </div>
            <div v-if="job.isRemote" class="text-xs font-medium text-blue-800 bg-blue-100 px-3 py-1 rounded-full">
              Remoto
            </div>
          </div>

          <p class="text-sm text-gray-700 mb-4 line-clamp-3">{{ job.description }}</p>

          <div class="flex flex-wrap gap-2 mb-4">
            <span
              v-for="tag in job.tags"
              :key="tag"
              class="bg-gray-200 text-gray-700 px-3 py-1 text-xs rounded-full"
            >#{{ tag }}</span>
          </div>

          <div class="text-sm font-medium text-gray-800 mb-4 flex flex-wrap gap-3">
            💰 {{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}/{{ job.salaryRange?.type }}
            <span class="text-gray-400">·</span>
            ⏳ {{ job.duration || 'Sin duración' }}
          </div>

          <router-link
            :to="`/jobs/${job._id}`"
            class="inline-block bg-emerald-600 text-white text-sm px-5 py-2 rounded-md hover:bg-emerald-700 transition"
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
