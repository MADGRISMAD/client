<template>
  <DefaultLayout>
    <div class="max-w-6xl mx-auto mt-8 px-4">
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-semibold text-gray-900">Ofertas disponibles</h1>
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-500">{{ filteredJobs.length }} ofertas</span>
          <span class="text-gray-300">|</span>
          <button
            @click="showFilters = !showFilters"
            class="text-sm text-emerald-600 hover:text-emerald-700 flex items-center gap-1"
          >
            <span><PhSlidersHorizontal class="ic" /></span>
            Filtros
          </button>
        </div>
      </div>

      <!-- Buscador y Filtros -->
      <div class="bg-white rounded-lg border border-gray-200 shadow-sm mb-6">
        <div class="p-4 border-b border-gray-100">
          <form @submit.prevent="searchWithAi" class="flex gap-2">
            <input
              v-model="search"
              type="text"
              placeholder="Busca por título o describe lo que quieres"
              class="w-full border border-gray-200 px-4 py-2 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-colors"
            />
            <button
              type="submit"
              :disabled="aiLoading || search.trim().length < 2"
              class="shrink-0 bg-emerald-600 text-white text-sm px-4 py-2 rounded-lg hover:bg-emerald-700 disabled:opacity-50 transition"
            >
              <PhSparkle class="ic" weight="fill" /> {{ aiLoading ? 'Buscando…' : 'Buscar con IA' }}
            </button>
          </form>
          <p v-if="aiResults" class="mt-2 text-xs text-gray-500">
            Resultados por significado, de más a menos parecido.
            <button type="button" @click="aiResults = null" class="text-emerald-600 hover:text-emerald-700 underline">Ver todas las ofertas</button>
          </p>
          <p v-if="aiError" class="mt-2 text-xs text-red-600">{{ aiError }}</p>
        </div>

        <div v-if="showFilters" class="p-4 border-b border-gray-100">
          <div class="flex flex-wrap gap-2">
            <button
              @click="filters.remote = !filters.remote"
              :class="['px-3 py-1.5 rounded-lg border text-sm transition min-w-[100px] flex items-center justify-center', filters.remote ? 'bg-emerald-50 text-emerald-700 border-emerald-200' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
            >
              <span class="mr-1"><PhMapPin class="ic" /></span>
              Remoto
            </button>
            <button
              @click="filters.freelance = !filters.freelance"
              :class="['px-3 py-1.5 rounded-lg border text-sm transition min-w-[100px] flex items-center justify-center', filters.freelance ? 'bg-emerald-50 text-emerald-700 border-emerald-200' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
            >
              <span class="mr-1"><PhLaptop class="ic" /></span>
              Freelance
            </button>
            <button
              @click="filters.project = !filters.project"
              :class="['px-3 py-1.5 rounded-lg border text-sm transition min-w-[100px] flex items-center justify-center', filters.project ? 'bg-emerald-50 text-emerald-700 border-emerald-200' : 'hover:bg-gray-50 border-gray-200 text-gray-600']"
            >
              <span class="mr-1"><PhPackage class="ic" /></span>
              Proyecto
            </button>
          </div>
        </div>
      </div>

      <div v-if="filteredJobs.length === 0" class="text-center py-12">
        <div class="text-gray-400 mb-2"><PhMagnifyingGlass class="ic" /></div>
        <p class="text-gray-500">No se encontraron ofertas que coincidan con tu búsqueda.</p>
        <button
          @click="resetFilters"
          class="mt-4 text-sm text-emerald-600 hover:text-emerald-700"
        >
          Limpiar filtros
        </button>
      </div>

      <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="job in filteredJobs"
          :key="job._id"
          :class="['bg-white rounded-lg border transition p-4 hover:shadow-md flex flex-col', job.highlighted ? 'md:col-span-2 border-emerald-200 bg-gradient-to-br from-white to-emerald-50' : 'border-gray-200']"
        >
          <div class="flex justify-between items-start mb-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 bg-emerald-100 rounded-lg flex items-center justify-center text-base font-semibold text-emerald-600">
                {{ job.company?.[0] || job.createdBy?.fullName?.[0] || '?' }}
              </div>
              <div>
                <div v-if="job.highlighted" class="text-xs font-medium text-emerald-700 bg-emerald-100 rounded-lg px-2 py-0.5 mb-1 inline-block">
                  <PhStar class="ic" /> Destacado
                </div>
                <h3 class="text-base font-semibold text-gray-900">{{ job.title }}</h3>
                <p class="text-sm text-gray-500">{{ job.company || job.createdBy?.fullName }}</p>
              </div>
            </div>
            <div class="flex flex-col items-end gap-1">
              <div v-if="job.matchScore != null" class="text-xs font-semibold text-emerald-700 bg-emerald-100 px-2 py-0.5 rounded-lg">
                {{ job.matchScore }}% de coincidencia
              </div>
              <div v-if="job.isRemote" class="text-xs font-medium text-emerald-700 bg-emerald-50 px-2 py-0.5 rounded-lg">
                Remoto
              </div>
            </div>
          </div>

          <p class="text-sm text-gray-600 mb-3" :class="{'line-clamp-2': !job.highlighted, 'line-clamp-4': job.highlighted}">{{ job.description }}</p>

          <div class="flex flex-wrap gap-1.5 mb-3">
            <span
              v-for="tag in job.tags"
              :key="tag"
              class="bg-gray-100 text-gray-600 px-2 py-0.5 text-xs rounded-lg"
            >#{{ tag }}</span>
          </div>

          <div class="flex items-center gap-2 text-sm text-gray-600 mb-3">
            <span class="flex items-center gap-1">
              <PhCoins class="ic" /> {{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}/{{ job.salaryRange?.type }}
            </span>
            <span class="text-gray-300">·</span>
            <span class="flex items-center gap-1">
              <PhHourglassMedium class="ic" /> {{ job.duration || 'Sin duración' }}
            </span>
          </div>

          <div class="mt-auto">
            <router-link
              :to="`/jobs/${job._id}`"
              class="inline-block w-full text-center bg-emerald-600 text-white text-sm px-4 py-2 rounded-lg hover:bg-emerald-700 transition"
            >
              Ver detalles
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { PhCoins, PhHourglassMedium, PhLaptop, PhMagnifyingGlass, PhMapPin, PhPackage, PhSlidersHorizontal, PhSparkle, PhStar } from '@phosphor-icons/vue'
import { onMounted, ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import AiService from '../services/AiService'
import AuthService from '../services/AuthService'

const route = useRoute()
const jobs = ref([])
const aiResults = ref(null)
const aiLoading = ref(false)
const aiError = ref('')
const search = ref(route.query.q || '')
const showFilters = ref(false)
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
  // la búsqueda de la portada llega con ?q=...&ai=1: con sesión se hace con IA; sin sesión queda el filtro normal
  if (route.query.ai && search.value.trim().length >= 2 && AuthService.isLoggedIn()) searchWithAi()
})

const searchWithAi = async () => {
  aiError.value = ''
  if (!AuthService.isLoggedIn()) {
    aiError.value = 'Inicia sesión para buscar con IA.'
    return
  }
  aiLoading.value = true
  try {
    // se ocultan las que casi no se parecen (menos de 10 %)
    aiResults.value = (await AiService.search(search.value.trim())).filter(j => j.matchScore >= 10)
  } catch (err) {
    aiError.value = err.response?.data?.message || 'No se pudo buscar con IA. Inténtalo de nuevo.'
  } finally {
    aiLoading.value = false
  }
}

const resetFilters = () => {
  aiResults.value = null
  search.value = ''
  filters.value = {
    remote: false,
    freelance: false,
    project: false
  }
}

const filteredJobs = computed(() => {
  return (aiResults.value ?? jobs.value).filter(job => {
    const textMatch =
      aiResults.value !== null ||
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
