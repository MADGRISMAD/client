<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50">
      <!-- HERO -->
      <section class="relative py-12 md:py-20 px-4 overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-br from-emerald-50 to-white opacity-50"></div>
        <div class="max-w-7xl mx-auto relative">
          <div class="grid lg:grid-cols-2 gap-8 md:gap-12 items-center">
            <div>
              <div class="inline-block bg-emerald-50 text-emerald-700 px-3 md:px-4 py-1 rounded-full text-xs md:text-sm font-medium mb-4 md:mb-6">
                🎮 ¡Bienvenido a la aventura profesional!
              </div>
              <h1 class="text-3xl md:text-4xl lg:text-5xl font-bold mb-4 md:mb-6 leading-tight">
                Conectando <span class="text-emerald-600">talento universitario</span> con oportunidades laborales
              </h1>
              <p class="text-lg md:text-xl text-gray-600 mb-6 md:mb-8">
                Encuentra trabajos freelance, remotos o por proyecto exclusivos para estudiantes universitarios
              </p>
              
              <!-- Search bar -->
              <div class="bg-white rounded-xl border border-gray-200 shadow-lg overflow-hidden">
                <div class="p-4 md:p-6">
                  <div class="relative">
                    <div class="absolute inset-y-0 left-0 pl-4 md:pl-6 flex items-center pointer-events-none">
                      <span class="text-gray-400 text-lg md:text-xl">🔍</span>
                    </div>
                    <input
                      v-model="search"
                      type="text"
                      placeholder="Buscar por título, descripción o etiquetas..."
                      class="w-full border-0 pl-12 md:pl-14 pr-4 md:pr-6 py-3 md:py-4 text-base md:text-lg focus:outline-none focus:ring-0"
                    />
                  </div>
                </div>
                <div class="p-4 md:p-6 border-t border-gray-100 bg-gray-50">
                  <div class="flex flex-wrap gap-2 md:gap-3">
                    <button
                      @click="filters.remote = !filters.remote"
                      :class="['px-4 md:px-6 py-2 md:py-3 rounded-lg text-sm md:text-base transition min-w-[100px] md:min-w-[120px] flex items-center justify-center gap-2', filters.remote ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'hover:bg-gray-100 border border-gray-200 text-gray-600']"
                    >
                      <span class="text-base md:text-lg">📍</span>
                      Remoto
                    </button>
                    <button
                      @click="filters.highlighted = !filters.highlighted"
                      :class="['px-4 md:px-6 py-2 md:py-3 rounded-lg text-sm md:text-base transition min-w-[100px] md:min-w-[120px] flex items-center justify-center gap-2', filters.highlighted ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'hover:bg-gray-100 border border-gray-200 text-gray-600']"
                    >
                      <span class="text-base md:text-lg">🌟</span>
                      Destacados
                    </button>
                    <button
                      @click="filters.project = !filters.project"
                      :class="['px-4 md:px-6 py-2 md:py-3 rounded-lg text-sm md:text-base transition min-w-[100px] md:min-w-[120px] flex items-center justify-center gap-2', filters.project ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'hover:bg-gray-100 border border-gray-200 text-gray-600']"
                    >
                      <span class="text-base md:text-lg">📦</span>
                      Proyecto
                    </button>
                  </div>
                </div>
              </div>
            </div>
            <div class="hidden lg:block">
              <div class="relative">
                <div class="absolute inset-0 bg-gradient-to-br from-emerald-100 to-emerald-50 rounded-2xl transform rotate-6"></div>
                <div class="relative bg-white rounded-2xl p-8 shadow-xl">
                  <div class="flex items-center gap-4 mb-6">
                    <div class="h-16 w-16 bg-emerald-100 rounded-xl flex items-center justify-center text-2xl font-semibold text-emerald-600">
                      U
                    </div>
                    <div>
                      <div class="text-sm font-medium text-emerald-700 bg-emerald-50 px-3 py-1 rounded-lg inline-block mb-1">
                        🌟 Destacado
                      </div>
                      <h3 class="text-xl font-semibold text-gray-900">Desarrollador Frontend</h3>
                      <p class="text-gray-500">Universidad Nacional</p>
                    </div>
                  </div>
                  <p class="text-gray-600 mb-6">Buscamos un desarrollador frontend con experiencia en Vue.js para un proyecto de 6 meses.</p>
                  <div class="flex flex-wrap gap-2 mb-6">
                    <span class="bg-gray-100 text-gray-600 px-3 py-1 text-sm rounded-lg">#Vue</span>
                    <span class="bg-gray-100 text-gray-600 px-3 py-1 text-sm rounded-lg">#JavaScript</span>
                    <span class="bg-gray-100 text-gray-600 px-3 py-1 text-sm rounded-lg">#Frontend</span>
                  </div>
                  <div class="flex items-center gap-4 text-gray-600">
                    <span class="flex items-center gap-2">
                      💰 $15 - $20/hora
                    </span>
                    <span class="text-gray-300">·</span>
                    <span class="flex items-center gap-2">
                      ⏳ 6 meses
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- OFERTAS DESTACADAS -->
      <section v-if="filteredJobs.length" class="py-12 md:py-20 px-4">
        <div class="max-w-7xl mx-auto">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8 md:mb-12">
            <div class="flex items-center gap-3 md:gap-4">
              <h2 class="text-2xl md:text-3xl font-bold text-gray-900">Ofertas destacadas</h2>
              <span class="bg-emerald-100 text-emerald-700 px-3 md:px-4 py-1 rounded-full text-xs md:text-sm font-medium">
                {{ filteredJobs.length }} disponibles
              </span>
            </div>
            <router-link
              to="/jobs"
              class="text-emerald-600 hover:text-emerald-700 flex items-center gap-2 text-base md:text-lg"
            >
              Ver todas
              <span class="text-lg md:text-xl">→</span>
            </router-link>
          </div>
          <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
            <div
              v-for="job in filteredJobs"
              :key="job._id"
              :class="['bg-white rounded-2xl border transition p-6 hover:shadow-xl flex flex-col', job.highlighted ? 'md:col-span-2 border-emerald-200 bg-gradient-to-br from-white to-emerald-50' : 'border-gray-200']"
            >
              <div class="flex justify-between items-start mb-4">
                <div class="flex items-center gap-4">
                  <div class="h-12 w-12 bg-emerald-100 rounded-xl flex items-center justify-center text-lg font-semibold text-emerald-600">
                    {{ job.company?.[0] || job.createdBy?.fullName?.[0] || '?' }}
                  </div>
                  <div>
                    <div v-if="job.highlighted" class="text-sm font-medium text-emerald-700 bg-emerald-100 rounded-lg px-3 py-1 mb-2 inline-block">
                      🌟 Destacado
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900">{{ job.title }}</h3>
                    <p class="text-gray-500">{{ job.company || job.createdBy?.fullName }}</p>
                  </div>
                </div>
                <div v-if="job.isRemote" class="text-sm font-medium text-emerald-700 bg-emerald-50 px-3 py-1 rounded-lg">
                  Remoto
                </div>
              </div>

              <p class="text-gray-600 mb-6" :class="{'line-clamp-2': !job.highlighted, 'line-clamp-4': job.highlighted}">{{ job.description }}</p>

              <div class="flex flex-wrap gap-2 mb-6">
                <span
                  v-for="tag in job.tags"
                  :key="tag"
                  class="bg-gray-100 text-gray-600 px-3 py-1 text-sm rounded-lg"
                >#{{ tag }}</span>
              </div>

              <div class="flex items-center gap-4 text-gray-600 mb-6">
                <span class="flex items-center gap-2">
                  💰 {{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}/{{ job.salaryRange?.type }}
                </span>
                <span class="text-gray-300">·</span>
                <span class="flex items-center gap-2">
                  ⏳ {{ job.duration || 'Sin duración' }}
                </span>
              </div>

              <div class="mt-auto">
                <router-link
                  :to="`/jobs/${job._id}`"
                  class="inline-block w-full text-center bg-emerald-600 text-white text-lg px-6 py-3 rounded-xl hover:bg-emerald-700 transition"
                >
                  Ver detalles
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- ¿CÓMO FUNCIONA? -->
      <section class="bg-white py-12 md:py-20 px-4">
        <div class="max-w-7xl mx-auto">
          <div class="text-center mb-8 md:mb-16">
            <div class="inline-block bg-emerald-50 text-emerald-700 px-3 md:px-4 py-1 rounded-full text-xs md:text-sm font-medium mb-4 md:mb-6">
              🎯 Tu camino al éxito
            </div>
            <h2 class="text-2xl md:text-3xl font-bold mb-4 md:mb-6">¿Cómo funciona?</h2>
            <p class="text-lg md:text-xl text-gray-600 max-w-3xl mx-auto">
              Sigue estos pasos para comenzar tu aventura profesional y destacar en el mundo laboral
            </p>
          </div>
          <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-8">
            <div class="bg-gray-50 rounded-2xl p-8 text-center relative overflow-hidden">
              <div class="absolute top-0 right-0 bg-emerald-100 text-emerald-700 px-4 py-1 rounded-bl-2xl text-sm font-medium">
                Nivel 1
              </div>
              <div class="text-4xl mb-6">📝</div>
              <h3 class="text-xl font-semibold mb-4">Crea tu perfil</h3>
              <p class="text-gray-600">Regístrate con tu correo .edu y completa tu perfil profesional.</p>
            </div>
            <div class="bg-gray-50 rounded-2xl p-8 text-center relative overflow-hidden">
              <div class="absolute top-0 right-0 bg-emerald-100 text-emerald-700 px-4 py-1 rounded-bl-2xl text-sm font-medium">
                Nivel 2
              </div>
              <div class="text-4xl mb-6">🔍</div>
              <h3 class="text-xl font-semibold mb-4">Explora vacantes</h3>
              <p class="text-gray-600">Filtra por tipo de trabajo, tags o modalidad. Aplica fácilmente.</p>
            </div>
            <div class="bg-gray-50 rounded-2xl p-8 text-center relative overflow-hidden">
              <div class="absolute top-0 right-0 bg-emerald-100 text-emerald-700 px-4 py-1 rounded-bl-2xl text-sm font-medium">
                Nivel 3
              </div>
              <div class="text-4xl mb-6">🚀</div>
              <h3 class="text-xl font-semibold mb-4">Gana experiencia</h3>
              <p class="text-gray-600">Colabora con empresas reales, mejora tu portafolio y obtén feedback.</p>
            </div>
          </div>
        </div>
      </section>

      <!-- RANKING DE TALENTO -->
      <section class="py-12 md:py-20 px-4">
        <div class="max-w-7xl mx-auto">
          <div class="text-center mb-8 md:mb-16">
            <div class="inline-block bg-emerald-50 text-emerald-700 px-3 md:px-4 py-1 rounded-full text-xs md:text-sm font-medium mb-4 md:mb-6">
              🏆 Compite y gana
            </div>
            <h2 class="text-2xl md:text-3xl font-bold mb-4 md:mb-6">Ranking de Talento</h2>
            <p class="text-lg md:text-xl text-gray-600 max-w-3xl mx-auto">
              Reconocemos a quienes destacan por su talento, dedicación y colaboración.
              Participa, suma puntos y sube en los rankings por desempeño individual, universidad y especialidad.
            </p>
          </div>

          <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-8">
            <!-- Estudiantes -->
            <div class="bg-white rounded-2xl border border-gray-200 p-8">
              <div class="flex items-center justify-between mb-6">
                <h3 class="text-xl font-semibold text-emerald-700">Top Estudiantes</h3>
                <span class="text-sm font-medium text-emerald-600 bg-emerald-50 px-3 py-1 rounded-lg">
                  🎮 Jugadores
                </span>
              </div>
              <ul class="space-y-6">
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      CT
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Camila Torres</p>
                      <p class="text-sm text-gray-500">Ingeniería de Software</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🏆</span>
                    850 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      IM
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Iván Martínez</p>
                      <p class="text-sm text-gray-500">Diseño UX/UI</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥈</span>
                    770 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      LG
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Lucía Gómez</p>
                      <p class="text-sm text-gray-500">Marketing Digital</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥉</span>
                    740 pts
                  </span>
                </li>
              </ul>
            </div>

            <!-- Universidades -->
            <div class="bg-white rounded-2xl border border-gray-200 p-8">
              <div class="flex items-center justify-between mb-6">
                <h3 class="text-xl font-semibold text-emerald-700">Top Universidades</h3>
                <span class="text-sm font-medium text-emerald-600 bg-emerald-50 px-3 py-1 rounded-lg">
                  🏫 Equipos
                </span>
              </div>
              <ul class="space-y-6">
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      U
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">UNAM</p>
                      <p class="text-sm text-gray-500">Ciudad de México</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🏆</span>
                    4,320 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      I
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">IPN</p>
                      <p class="text-sm text-gray-500">México</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥈</span>
                    3,980 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      G
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">UDG</p>
                      <p class="text-sm text-gray-500">Guadalajara</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥉</span>
                    3,600 pts
                  </span>
                </li>
              </ul>
            </div>

            <!-- Especialidades -->
            <div class="bg-white rounded-2xl border border-gray-200 p-8">
              <div class="flex items-center justify-between mb-6">
                <h3 class="text-xl font-semibold text-emerald-700">Top Especialidades</h3>
                <span class="text-sm font-medium text-emerald-600 bg-emerald-50 px-3 py-1 rounded-lg">
                  🎯 Categorías
                </span>
              </div>
              <ul class="space-y-6">
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      IS
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Ingeniería de Software</p>
                      <p class="text-sm text-gray-500">Tecnología</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🏆</span>
                    7,820 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      DG
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Diseño Gráfico</p>
                      <p class="text-sm text-gray-500">Creatividad</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥈</span>
                    6,150 pts
                  </span>
                </li>
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                      MD
                    </div>
                    <div>
                      <p class="text-lg font-medium text-gray-900">Marketing Digital</p>
                      <p class="text-sm text-gray-500">Comercial</p>
                    </div>
                  </div>
                  <span class="text-emerald-600 font-medium flex items-center gap-2">
                    <span class="text-xl">🥉</span>
                    5,940 pts
                  </span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      <!-- CTA PARA EMPRESAS -->
      <section
        v-if="isLoggedIn() && user.value?.role === 'employer'"
        class="bg-emerald-50 py-12 md:py-20 px-4"
      >
        <div class="max-w-7xl mx-auto text-center">
          <div class="inline-block bg-emerald-100 text-emerald-700 px-3 md:px-4 py-1 rounded-full text-xs md:text-sm font-medium mb-4 md:mb-6">
            🎮 Únete a la aventura
          </div>
          <h2 class="text-2xl md:text-3xl font-bold text-emerald-700 mb-4 md:mb-6">¿Eres una empresa?</h2>
          <p class="text-lg md:text-xl text-gray-600 mb-6 md:mb-8 max-w-3xl mx-auto">
            Publica vacantes y encuentra talento universitario motivado y con ganas de aprender.
          </p>
          <router-link
            to="/jobs/create"
            class="inline-block bg-emerald-600 text-white px-6 md:px-8 py-3 md:py-4 rounded-xl text-base md:text-lg font-medium hover:bg-emerald-700 transition"
          >
            Publicar vacante
          </router-link>
        </div>
      </section>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import { useAuth } from '../composables/useAuth'

const search = ref('')
const jobs = ref([])
const loading = ref(true)
const error = ref('')
const { user, isLoggedIn } = useAuth()

const filters = ref({
  remote: false,
  highlighted: false,
  project: false
})

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
  let filtered = jobs.value

  if (search.value) {
    filtered = filtered.filter(job =>
      job.title.toLowerCase().includes(search.value.toLowerCase()) ||
      job.description.toLowerCase().includes(search.value.toLowerCase()) ||
      job.tags?.some(tag => tag.toLowerCase().includes(search.value.toLowerCase()))
    )
  }

  if (filters.value.remote) {
    filtered = filtered.filter(job => job.isRemote)
  }

  if (filters.value.highlighted) {
    filtered = filtered.filter(job => job.highlighted)
  }

  if (filters.value.project) {
    filtered = filtered.filter(job => job.salaryRange?.type === 'proyecto')
  }

  return filtered
})
</script>
