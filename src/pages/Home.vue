<template>
  <DefaultLayout>
    <div class="bg-white text-gray-800">
      <!-- HERO -->
      <section class="text-center py-16 px-6">
        <h1 class="text-4xl md:text-5xl font-bold mb-4 leading-tight">
          Conectando <span class="text-green-600">talento universitario</span> con oportunidades laborales
        </h1>
        <p class="text-gray-600 text-lg mb-8">
          Encuentra trabajos freelance, remotos o por proyecto exclusivos para estudiantes universitarios
        </p>

        <!-- Search bar -->
        <div class="max-w-2xl mx-auto flex flex-wrap gap-2 justify-center mb-6">
          <input
            v-model="search"
            type="text"
            placeholder="🔍 Buscar oportunidades…"
            class="flex-1 min-w-[250px] border px-4 py-2 rounded-md text-sm"
          />
          <button class="bg-green-600 text-white px-6 py-2 rounded-md hover:bg-green-700">
            Buscar
          </button>
        </div>

        <!-- Filtros -->
        <div class="flex flex-wrap gap-2 justify-center text-sm mb-4">
          <button class="border px-3 py-1 rounded-full hover:bg-gray-50">📍 Remoto</button>
          <button class="border px-3 py-1 rounded-full hover:bg-gray-50">💻 Freelance</button>
          <button class="border px-3 py-1 rounded-full hover:bg-gray-50">📦 Proyecto</button>
          <button class="border px-3 py-1 rounded-full hover:bg-gray-50">🕓 Medio tiempo</button>
        </div>
      </section>

      <!-- Jobs destacados -->
      <section class="max-w-6xl mx-auto px-6 mb-20">
        <h2 class="text-2xl font-bold mb-6">Ofertas destacadas</h2>
        <div class="grid md:grid-cols-2 gap-6">
          <!-- CARD DE TRABAJO MOCK -->
          <!-- CARD REAL -->
<div
  v-for="job in filteredJobs"
  :key="job._id"
  class="border p-6 rounded-lg shadow-sm hover:shadow transition"
>
  <div class="flex items-center justify-between mb-2">
    <div
      v-if="job.highlighted"
      class="text-green-600 text-xs font-semibold bg-green-100 px-2 py-0.5 rounded-full"
    >Destacado</div>
    <div
      v-if="job.isRemote"
      class="text-xs text-blue-600 bg-blue-100 px-2 py-0.5 rounded-full"
    >Remoto</div>
  </div>

  <h3 class="text-lg font-semibold">{{ job.title }}</h3>
  <p class="text-gray-500 text-sm mb-2">{{ job.company }}</p>
  <p class="text-sm text-gray-700 mb-3 line-clamp-2">{{ job.description }}</p>

  <div class="flex flex-wrap gap-2 mb-3">
    <span
      v-for="tag in job.tags"
      :key="tag"
      class="bg-gray-100 text-gray-600 px-2 py-1 text-xs rounded-full"
    >
      {{ tag }}
    </span>
  </div>

  <p class="text-sm font-medium mb-3">
    {{ job.salaryRange?.min }}-{{ job.salaryRange?.max }}/{{ job.salaryRange?.type }} · {{ job.duration }}
  </p>

  <router-link
    :to="`/jobs/${job._id}`"
    class="inline-block bg-green-600 text-white text-sm px-4 py-2 rounded-md hover:bg-green-700"
  >
    Ver detalles
  </router-link>
</div>

        </div>
      </section>

      <!-- ¿Cómo funciona? -->
      <section class="bg-gray-50 py-20 px-6">
        <div class="max-w-6xl mx-auto text-center">
          <h2 class="text-3xl font-bold mb-12">¿Cómo funciona?</h2>
          <div class="grid md:grid-cols-3 gap-10">
            <div>
              <div class="text-4xl mb-3">📝</div>
              <h3 class="text-xl font-semibold mb-2">Crea tu perfil</h3>
              <p class="text-gray-600">Regístrate con tu correo .edu y completa tu perfil profesional.</p>
            </div>
            <div>
              <div class="text-4xl mb-3">🔍</div>
              <h3 class="text-xl font-semibold mb-2">Explora vacantes</h3>
              <p class="text-gray-600">Filtra por tipo de trabajo, tags o modalidad. Aplica fácilmente.</p>
            </div>
            <div>
              <div class="text-4xl mb-3">🚀</div>
              <h3 class="text-xl font-semibold mb-2">Gana experiencia</h3>
              <p class="text-gray-600">Colabora con empresas reales, mejora tu portafolio y obtén feedback.</p>
            </div>
          </div>
        </div>
      </section>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'

const search = ref('')
const jobs = ref([])
const loading = ref(true)
const error = ref('')

onMounted(async () => {
  try {
    const data = await JobService.getAll()
    jobs.value = data
  } catch (err) {
    error.value = 'Error al cargar vacantes'
    console.error(err)
  } finally {
    loading.value = false
  }
})

const filteredJobs = computed(() => {
  if (!search.value) return jobs.value
  return jobs.value.filter(job =>
    job.title.toLowerCase().includes(search.value.toLowerCase()) ||
    job.description.toLowerCase().includes(search.value.toLowerCase()) ||
    job.tags?.some(tag => tag.toLowerCase().includes(search.value.toLowerCase()))
  )
})
</script>

