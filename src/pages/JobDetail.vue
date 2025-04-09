<template>
  <DefaultLayout>
    <div class="bg-gradient-to-b from-white to-gray-50 min-h-screen">
      <div class="max-w-7xl mx-auto py-10 px-4">
        <!-- Back & Share -->
        <div class="flex justify-between items-center mb-6">
          <router-link to="/jobs" class="flex items-center gap-2 text-sm text-muted-foreground hover:text-emerald-600">
            <span class="material-icons text-base">arrow_back</span>
            Volver a resultados
          </router-link>
          <div class="flex gap-4">
            <button class="flex items-center gap-1 text-sm text-muted-foreground hover:text-gray-700">
              <span class="material-icons text-base">star_border</span>
              Guardar
            </button>
            <button class="flex items-center gap-1 text-sm text-muted-foreground hover:text-gray-700">
              <span class="material-icons text-base">share</span>
              Compartir
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Main Content -->
          <div class="lg:col-span-2 space-y-8">
            <div class="flex gap-4 items-start">
              <div class="h-16 w-16 bg-emerald-100 rounded-lg flex items-center justify-center">
                <span class="text-2xl font-bold text-emerald-600">{{ job?.company?.[0] || 'E' }}</span>
              </div>
              <div class="flex-1">
                <h1 class="text-2xl font-bold text-gray-900">{{ job?.title }}</h1>
                <p class="text-muted-foreground">{{ job?.company || 'Empresa desconocida' }}</p>

                <div class="flex flex-wrap gap-4 mt-3 text-sm text-muted-foreground">
                  <span v-if="job?.isRemote" class="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs">Remoto</span>
                  <span>Publicado: {{ publishedAgo }}</span>
                  <span>{{ job?.applicants?.length || 0 }} aplicantes</span>
                </div>

                <div class="flex flex-wrap gap-2 mt-3">
                  <span
                    v-for="tag in job?.tags || []"
                    :key="tag"
                    class="bg-gray-200 text-gray-800 px-3 py-1 text-xs rounded-full"
                  >
                    #{{ tag }}
                  </span>
                </div>
              </div>
            </div>

            <div class="bg-white border border-gray-100 rounded-xl shadow-sm p-6">
              <h2 class="text-lg font-semibold mb-3 text-gray-900">Descripción del puesto</h2>
              <p class="text-gray-700 whitespace-pre-line text-sm">{{ job?.description }}</p>
            </div>

            <div
              v-if="job?.responsibilities?.length"
              class="bg-white border border-gray-100 rounded-xl shadow-sm p-6"
            >
              <h2 class="text-lg font-semibold mb-3 text-gray-900">Responsabilidades</h2>
              <ul class="list-disc ml-5 space-y-2 text-sm text-gray-700">
                <li v-for="(resp, idx) in job.responsibilities" :key="idx">{{ resp }}</li>
              </ul>
            </div>

            <div
              v-if="job?.requirements?.length"
              class="bg-white border border-gray-100 rounded-xl shadow-sm p-6"
            >
              <h2 class="text-lg font-semibold mb-3 text-gray-900">Requisitos</h2>
              <ul class="list-disc ml-5 space-y-2 text-sm text-gray-700">
                <li v-for="(req, idx) in job.requirements" :key="idx">{{ req }}</li>
              </ul>
            </div>

            <div
              v-if="job?.benefits?.length"
              class="bg-white border border-gray-100 rounded-xl shadow-sm p-6"
            >
              <h2 class="text-lg font-semibold mb-3 text-gray-900">Beneficios</h2>
              <ul class="list-disc ml-5 space-y-2 text-sm text-gray-700">
                <li v-for="(benefit, idx) in job.benefits" :key="idx">{{ benefit }}</li>
              </ul>
            </div>

            <div class="bg-white border border-gray-100 rounded-xl shadow-sm p-6">
              <h2 class="text-lg font-semibold mb-3 text-gray-900">Trabajos similares</h2>
              <div class="space-y-4 text-sm">
                <div class="border border-gray-100 rounded-lg p-4 hover:bg-gray-50 transition">
                  <h3 class="font-semibold">Desarrollador Frontend Vue.js</h3>
                  <p class="text-muted-foreground">DigitalNew - Remoto</p>
                </div>
                <div class="border border-gray-100 rounded-lg p-4 hover:bg-gray-50 transition">
                  <h3 class="font-semibold">UX/UI Designer Junior</h3>
                  <p class="text-muted-foreground">CreativeStudio - Híbrido</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <aside v-if="canApply" class="space-y-4 h-fit lg:sticky lg:top-6">
            <div class="bg-white border border-gray-100 rounded-xl shadow-sm p-6">
              <h2 class="text-lg font-semibold mb-4 text-gray-900">Aplicar a esta vacante</h2>
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
                <label class="block text-sm mb-1 font-medium text-gray-700">Carta de presentación</label>
                <textarea
                  v-model="coverLetter"
                  class="w-full border border-gray-200 rounded-md px-3 py-2 text-sm mb-3"
                  rows="4"
                  placeholder="Habla brevemente de tu interés y experiencia..."
                ></textarea>

                <button
                  @click="handleApply"
                  class="w-full bg-emerald-600 text-white py-2 rounded-md hover:bg-emerald-700 text-sm font-medium"
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

const canApply = computed(() => {
  if (!isLoggedIn()) return false
  if (user.value?.role !== 'student') return false
  if (!job.value) return false
  return job.value.createdBy !== user.value._id
})

onMounted(async () => {
  try {
    job.value = await JobService.getById(route.params.id)
  } catch (err) {
    error.value = 'No se pudo cargar la oferta'
    console.error(err)
  }
})

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

const publishedAgo = computed(() => {
  if (!job.value?.createdAt) return 'Desconocida'
  const created = new Date(job.value.createdAt)
  const now = new Date()
  const diffDays = Math.floor((now - created) / (1000 * 60 * 60 * 24))
  return diffDays === 0 ? 'hoy' : diffDays === 1 ? 'hace 1 día' : `hace ${diffDays} días`
})

const salaryFormatted = computed(() => {
  if (!job.value?.salaryRange) return 'No especificado'
  const s = job.value.salaryRange
  return `$${s.min}-${s.max}/${s.type} ${s.currency}`
})
</script>