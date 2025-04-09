<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50 text-gray-800">
      <!-- HERO -->
      <section class="text-center py-16 px-6">
        <h1 class="text-4xl md:text-5xl font-bold mb-4 leading-tight">
          Conectando <span class="text-emerald-600">talento universitario</span> con oportunidades laborales
        </h1>
        <p class="text-gray-600 text-lg mb-8 max-w-2xl mx-auto">
          Encuentra trabajos freelance, remotos o por proyecto exclusivos para estudiantes universitarios
        </p>

        <!-- Search bar -->
        <div class="max-w-2xl mx-auto flex flex-wrap gap-2 justify-center mb-6">
          <input
            v-model="search"
            type="text"
            placeholder="🔍 Buscar oportunidades…"
            class="flex-1 min-w-[250px] border border-gray-300 px-4 py-3 rounded-md text-sm focus:ring focus:ring-green-200"
          />
          <button class="bg-emerald-600 text-white px-6 py-3 rounded-md hover:bg-emerald-700">
            Buscar
          </button>
        </div>

        <!-- Filtros -->
        <div class="flex flex-wrap gap-2 justify-center text-sm mb-4">
          <button class="border border-gray-300 px-3 py-1.5 rounded-full hover:bg-gray-100">📍 Remoto</button>
          <button class="border border-gray-300 px-3 py-1.5 rounded-full hover:bg-gray-100">💻 Freelance</button>
          <button class="border border-gray-300 px-3 py-1.5 rounded-full hover:bg-gray-100">📦 Proyecto</button>
        </div>
      </section>

      <section v-if="filteredJobs.length" class="mb-20 max-w-7xl mx-auto mt-10 px-4">
        <h2 class="text-2xl font-bold mb-6 text-center">Ofertas destacadas</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="job in filteredJobs"
            :key="job._id"
            :class="[
              'bg-white rounded-xl border p-6 transition-shadow',
              job.highlighted ? 'sm:col-span-2 border-emerald-100 bg-gradient-to-br from-white to-emerald-50 hover:shadow-md' : 'border-gray-100 hover:shadow-sm'
            ]"
          >
            <div class="flex justify-between items-start">
              <div class="flex items-center gap-4">
                <div class="h-14 w-14  bg-gray-100 rounded-lg flex items-center justify-center text-lg font-semibold text-gray-400">
                  <span>{{ job.company?.charAt(0) || '?' }}</span>
                </div>
                <div>
                  <div v-if="job.highlighted" class="text-xs  font-semibold bg-emerald-100 text-emerald-800 rounded-full px-2 py-1 mb-1 inline-block">
                    Destacado
                  </div>
                  <h3 class="text-lg font-bold text-gray-800">{{ job.title }}</h3>
                  <p class="text-sm text-gray-500">{{ job.company || job.createdBy?.fullName }}</p>
                </div>
              </div>
              <div v-if="job.isRemote" class="text-xs font-medium bg-blue-50 text-blue-700 rounded-full px-2 py-1">
                Remoto
              </div>
            </div>

            <p class="mt-4 text-sm text-gray-600 line-clamp-3">
              {{ job.description }}
            </p>

            <div class="mt-4 flex flex-wrap gap-2">
              <span
                v-for="tag in job.tags"
                :key="tag"
                class="bg-gray-100 text-gray-700 px-3 py-1 text-xs rounded-full"
              >
                {{ tag }}
              </span>
            </div>

            <div class="mt-6 flex items-center justify-between">
              <div>
                <p class="text-sm font-medium">
                  💰 {{ job.salaryRange?.min }} - {{ job.salaryRange?.max }}/{{ job.salaryRange?.type }}
                </p>
                <p class="text-xs text-gray-400">⏳ {{ job.duration || 'Duración no especificada' }}</p>
              </div>
              <router-link
                :to="`/jobs/${job._id}`"
                class="bg-emerald-600 text-white text-sm px-4 py-2 rounded-md hover:bg-emerald-700 transition"
              >
                Ver detalles
              </router-link>
            </div>
          </div>
        </div>
      </section>

      <!-- ¿Cómo funciona? -->
      <section class="bg-white py-20 px-6">
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

      <!-- SECCIÓN DE GAMIFICACIÓN DESTACADA -->
<section class="bg-gradient-to-br from-white via-emerald-50 to-white py-20 px-6  ">
  <div class="max-w-7xl mx-auto text-center">
    <h2 class="text-4xl font-bold text-emerald-700 mb-4">Ranking de Talento</h2>
    <p class="text-gray-600 max-w-2xl mx-auto mb-12 text-lg">
      Reconocemos a quienes destacan por su talento, dedicación y colaboración.
      Participa, suma puntos y sube en los rankings por desempeño individual, universidad y especialidad.
    </p>

    <div class="grid md:grid-cols-3 gap-8 text-left">
      <!-- Estudiantes -->
      <div class="bg-white p-6 rounded-2xl shadow-md border border-gray-100">
        <h3 class="text-xl font-semibold text-emerald-700 mb-4">Top Estudiantes</h3>
        <ul class="space-y-4">
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <img src="https://i.pravatar.cc/40?img=11" class="w-10 h-10 rounded-full" />
              <div>
                <p class="font-medium text-gray-800">Camila Torres</p>
                <p class="text-xs text-gray-500">Ingeniería de Software</p>
              </div>
            </div>
            <span class="text-emerald-600 font-bold">🏆 850 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <img src="https://i.pravatar.cc/40?img=32" class="w-10 h-10 rounded-full" />
              <div>
                <p class="font-medium text-gray-800">Iván Martínez</p>
                <p class="text-xs text-gray-500">Diseño UX/UI</p>
              </div>
            </div>
            <span class="text-yellow-500 font-bold">🥈 770 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <img src="https://i.pravatar.cc/40?img=22" class="w-10 h-10 rounded-full" />
              <div>
                <p class="font-medium text-gray-800">Lucía Gómez</p>
                <p class="text-xs text-gray-500">Marketing Digital</p>
              </div>
            </div>
            <span class="text-amber-600 font-bold">🥉 740 pts</span>
          </li>
        </ul>
      </div>

      <!-- Universidades -->
      <div class="bg-white p-6 rounded-2xl shadow-md border border-gray-100">
        <h3 class="text-xl font-semibold text-emerald-700 mb-4">Top Universidades</h3>
        <ul class="space-y-4">
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-sm font-bold">
                U
              </div>
              <div>
                <p class="font-medium text-gray-800">UNAM</p>
                <p class="text-xs text-gray-500">Ciudad de México</p>
              </div>
            </div>
            <span class="text-emerald-600 font-bold">4,320 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-yellow-100 text-yellow-700 flex items-center justify-center text-sm font-bold">
                I
              </div>
              <div>
                <p class="font-medium text-gray-800">IPN</p>
                <p class="text-xs text-gray-500">México</p>
              </div>
            </div>
            <span class="text-yellow-500 font-bold">3,980 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-amber-100 text-amber-700 flex items-center justify-center text-sm font-bold">
                G
              </div>
              <div>
                <p class="font-medium text-gray-800">UDG</p>
                <p class="text-xs text-gray-500">Guadalajara</p>
              </div>
            </div>
            <span class="text-amber-600 font-bold">3,600 pts</span>
          </li>
        </ul>
      </div>

      <!-- Especialidades -->
      <div class="bg-white p-6 rounded-2xl shadow-md border border-gray-100">
        <h3 class="text-xl font-semibold text-emerald-700 mb-4">Top Especialidades</h3>
        <ul class="space-y-4">
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-emerald-100 text-emerald-700 flex items-center justify-center text-sm font-bold">
                IS
              </div>
              <div>
                <p class="font-medium text-gray-800">Ingeniería de Software</p>
                <p class="text-xs text-gray-500">Tecnología</p>
              </div>
            </div>
            <span class="text-emerald-600 font-bold">7,820 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-yellow-100 text-yellow-700 flex items-center justify-center text-sm font-bold">
                DG
              </div>
              <div>
                <p class="font-medium text-gray-800">Diseño Gráfico</p>
                <p class="text-xs text-gray-500">Creatividad</p>
              </div>
            </div>
            <span class="text-yellow-500 font-bold">6,150 pts</span>
          </li>
          <li class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="h-10 w-10 rounded-full bg-amber-100 text-amber-700 flex items-center justify-center text-sm font-bold">
                MD
              </div>
              <div>
                <p class="font-medium text-gray-800">Marketing Digital</p>
                <p class="text-xs text-gray-500">Comercial</p>
              </div>
            </div>
            <span class="text-amber-600 font-bold">5,940 pts</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</section>


      <!-- CTA PARA EMPRESAS -->
      <section
        v-if="isLoggedIn() && user.value?.role === 'employer'"
        class="bg-green-50 py-16 px-6 text-center"
      >
        <h2 class="text-3xl font-bold text-green-700 mb-4">¿Eres una empresa?</h2>
        <p class="text-gray-700 mb-6">
          Publica vacantes y encuentra talento universitario motivado y con ganas de aprender.
        </p>
        <router-link
          to="/jobs/create"
          class="bg-green-600 text-white px-6 py-3 rounded-md hover:bg-green-700 text-lg font-medium transition"
        >
          Publicar vacante
        </router-link>
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
