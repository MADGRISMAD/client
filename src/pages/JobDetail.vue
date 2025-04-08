<template>
  <DefaultLayout>
    <div class="max-w-7xl mx-auto py-6 px-4">
      <!-- Barra superior -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2 text-sm text-gray-500">
          <router-link to="/jobs" class="flex items-center gap-1 hover:underline">
            <span class="material-icons text-base">arrow_back</span>
            Volver a resultados
          </router-link>
        </div>
        <div class="flex items-center gap-4">
          <button class="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700">
            <span class="material-icons text-base">star_border</span>
            Guardar
          </button>
          <button class="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700">
            <span class="material-icons text-base">share</span>
            Compartir
          </button>
        </div>
      </div>

      <!-- GRID PRINCIPAL -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Columna izquierda (2/3) -->
        <div class="lg:col-span-2 space-y-6">
          <!-- ENCABEZADO DE LA VACANTE -->
          <section class="bg-white border border-gray-200 rounded-md p-6 shadow-sm">
            <h1 class="text-2xl font-bold mb-1 text-gray-800">{{ job?.title }}</h1>
            <p class="text-gray-600 text-sm mb-2">{{ job?.company || 'Empresa desconocida' }}</p>
            <div class="flex flex-wrap items-center gap-2 text-sm text-gray-500 mb-2">
              <span v-if="job?.isRemote" class="bg-green-50 text-green-600 px-2 py-1 rounded-full text-xs">
                Remoto
              </span>
              <span>
                • Publicado: {{ publishedAgo }} | {{ job?.applicants?.length || 0 }} aplicantes
              </span>
            </div>
            <div class="flex flex-wrap gap-2 mt-2">
              <span
                v-for="tag in job?.tags || []"
                :key="tag"
                class="bg-gray-100 text-gray-600 px-2 py-1 rounded-full text-xs"
              >
                {{ tag }}
              </span>
            </div>
          </section>

          <!-- SECCIÓN DESCRIPCIÓN -->
          <section class="bg-white border border-gray-200 rounded-md p-6 shadow-sm">
            <h2 class="text-lg font-semibold mb-3 text-gray-800">Descripción del puesto</h2>
            <p class="text-gray-700 whitespace-pre-line">
              {{ job?.description }}
            </p>
          </section>

          <!-- Responsabilidades (si existen) -->
          <section
            v-if="job?.responsibilities?.length"
            class="bg-white border border-gray-200 rounded-md p-6 shadow-sm"
          >
            <h2 class="text-lg font-semibold mb-3 text-gray-800">Responsabilidades</h2>
            <ul class="list-disc ml-5 space-y-1 text-gray-700">
              <li v-for="(resp, idx) in job.responsibilities" :key="idx">{{ resp }}</li>
            </ul>
          </section>

          <!-- Requisitos (si existen) -->
          <section
            v-if="job?.requirements?.length"
            class="bg-white border border-gray-200 rounded-md p-6 shadow-sm"
          >
            <h2 class="text-lg font-semibold mb-3 text-gray-800">Requisitos</h2>
            <ul class="list-disc ml-5 space-y-1 text-gray-700">
              <li v-for="(req, idx) in job.requirements" :key="idx">{{ req }}</li>
            </ul>
          </section>

          <!-- Beneficios (si existen) -->
          <section
            v-if="job?.benefits?.length"
            class="bg-white border border-gray-200 rounded-md p-6 shadow-sm"
          >
            <h2 class="text-lg font-semibold mb-3 text-gray-800">Beneficios</h2>
            <ul class="list-disc ml-5 space-y-1 text-gray-700">
              <li v-for="(benefit, idx) in job.benefits" :key="idx">{{ benefit }}</li>
            </ul>
          </section>

          <!-- Trabajos similares (Placeholder) -->
          <section class="bg-white border border-gray-200 rounded-md p-6 shadow-sm">
            <h2 class="text-lg font-semibold mb-3 text-gray-800">Trabajos similares</h2>
            <div class="space-y-4 text-sm">
              <div class="border border-gray-200 rounded-md p-4 hover:shadow-sm transition">
                <h3 class="font-semibold">Desarrollador Frontend Vue.js</h3>
                <p class="text-gray-500">DigitalNew - Remoto</p>
              </div>
              <div class="border border-gray-200 rounded-md p-4 hover:shadow-sm transition">
                <h3 class="font-semibold">UX/UI Designer Junior</h3>
                <p class="text-gray-500">CreativeStudio - Híbrido</p>
              </div>
            </div>
          </section>
        </div>

        <!-- Columna derecha: Card de aplicación -->
        <aside v-if="canApply" class="space-y-4 h-fit lg:sticky lg:top-6">
          <div class="bg-white border border-gray-200 rounded-md p-6 shadow-sm">
            <h2 class="text-lg font-semibold mb-4 text-gray-800">Aplicar a esta vacante</h2>
            <div class="text-sm space-y-3 mb-6 text-gray-700">
              <div class="flex justify-between">
                <span class="text-gray-500 font-medium">Compensación:</span>
                <span class="text-green-700 font-semibold">{{ salaryFormatted }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500 font-medium">Duración:</span>
                <span>{{ job?.duration || 'No especificada' }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-500 font-medium">Dedicación:</span>
                <span>{{ job?.dedication || 'No especificada' }}</span>
              </div>
            </div>

            <div>
              <label class="block text-sm mb-1 font-medium">Carta de presentación</label>
              <textarea
                v-model="coverLetter"
                class="w-full border border-gray-200 rounded px-3 py-2 text-sm mb-3"
                rows="4"
                placeholder="Habla brevemente de tu interés y experiencia..."
              ></textarea>

              <button
                @click="handleApply"
                class="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700"
              >
                Aplicar ahora
              </button>

              <p v-if="success" class="text-green-600 text-sm mt-2">{{ success }}</p>
              <p v-if="error" class="text-red-500 text-sm mt-2">{{ error }}</p>

              <p class="text-xs text-gray-400 mt-3">
                Al aplicar, aceptas nuestros <a href="#" class="underline">términos y condiciones</a>.
              </p>
            </div>
          </div>
        </aside>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuth } from '../composables/useAuth'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import axios from 'axios'

const route = useRoute()
const { user, isLoggedIn } = useAuth()

const job = ref(null)
const coverLetter = ref('')
const success = ref('')
const error = ref('')

// Determina si el usuario puede aplicar: Es estudiante y NO es el creador
const canApply = computed(() => {
  if (!isLoggedIn()) return false
  if (user.value?.role !== 'student') return false
  if (!job.value) return false
  return job.value.createdBy !== user.value._id
})

// Cargar la oferta
onMounted(async () => {
  try {
    job.value = await JobService.getById(route.params.id)
  } catch (err) {
    error.value = 'No se pudo cargar la oferta'
    console.error(err)
  }
})

// Aplicar a la oferta
const handleApply = async () => {
  error.value = ''
  success.value = ''
  try {
    const token = localStorage.getItem('token')
    await axios.post(
      `http://localhost:5000/api/jobs/${job.value._id}/apply`,
      { coverLetter: coverLetter.value },
      { headers: { Authorization: `Bearer ${token}` } }
    )
    success.value = 'Aplicación enviada con éxito.'
    coverLetter.value = ''
  } catch (err) {
    error.value = err.response?.data?.message || 'Error al aplicar.'
  }
}

// Formatear fecha para el encabezado
const publishedAgo = computed(() => {
  if (!job.value?.createdAt) return 'Desconocida'
  const created = new Date(job.value.createdAt)
  const now = new Date()
  const diffDays = Math.floor((now - created) / (1000 * 60 * 60 * 24))
  return diffDays === 0 ? 'hoy' : diffDays === 1 ? 'hace 1 día' : `hace ${diffDays} días`
})

// Formatear salario
const salaryFormatted = computed(() => {
  if (!job.value?.salaryRange) return 'No especificado'
  const s = job.value.salaryRange
  return `$${s.min}-${s.max}/${s.type} ${s.currency}`
})
</script>
