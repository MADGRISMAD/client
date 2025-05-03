<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 py-12">
        <!-- Encabezado -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
          <router-link 
            to="/jobs" 
            class="flex items-center gap-2 text-sm text-gray-600 hover:text-emerald-600 transition-colors"
          >
            <span class="text-lg">←</span>
            Volver a resultados
          </router-link>
          <div class="flex items-center gap-3">
            <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
              <span class="text-lg">⭐</span>
              Guardar
            </button>
            <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
              <span class="text-lg">🔗</span>
              Compartir
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Contenido principal -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Información principal -->
            <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6">
              <div class="flex gap-4 items-start">
                <div class="h-16 w-16 rounded-xl bg-emerald-100 flex items-center justify-center">
                  <span class="text-2xl font-bold text-emerald-600">{{ job?.company?.[0] || 'E' }}</span>
                </div>
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <h1 class="text-2xl font-bold text-gray-900">{{ job?.title }}</h1>
                    <span v-if="job?.highlighted" class="bg-emerald-100 text-emerald-700 px-3 py-1 rounded-full text-sm font-medium">
                      🌟 Destacado
                    </span>
                  </div>
                  <p class="text-gray-600">{{ job?.company || 'Empresa desconocida' }}</p>

                  <div class="flex flex-wrap gap-3 mt-4">
                    <div v-if="job?.isRemote" class="flex items-center gap-2 text-sm text-emerald-600">
                      <span class="text-lg">📍</span>
                      <span>Remoto</span>
                    </div>
                    <div class="flex items-center gap-2 text-sm text-gray-600">
                      <span class="text-lg">📅</span>
                      <span>Publicado {{ publishedAgo }}</span>
                    </div>
                    <div class="flex items-center gap-2 text-sm text-gray-600">
                      <span class="text-lg">👥</span>
                      <span>{{ job?.applicants?.length || 0 }} aplicantes</span>
                    </div>
                  </div>

                  <div class="flex flex-wrap gap-2 mt-4">
                    <span
                      v-for="tag in job?.tags || []"
                      :key="tag"
                      class="bg-gray-100 text-gray-600 px-3 py-1 text-sm rounded-lg"
                    >
                      #{{ tag }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Descripción -->
            <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6">
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">📝</span>
                <h2 class="text-xl font-semibold text-gray-900">Descripción del puesto</h2>
              </div>
              <p class="text-gray-600 whitespace-pre-line">{{ job?.description }}</p>
            </div>

            <!-- Responsabilidades -->
            <div
              v-if="job?.responsibilities?.length"
              class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6"
            >
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">🎯</span>
                <h2 class="text-xl font-semibold text-gray-900">Responsabilidades</h2>
              </div>
              <ul class="space-y-3">
                <li v-for="(resp, idx) in job.responsibilities" :key="idx" class="flex items-start gap-3 text-gray-600">
                  <span class="text-emerald-600 mt-1">•</span>
                  <span>{{ resp }}</span>
                </li>
              </ul>
            </div>

            <!-- Requisitos -->
            <div
              v-if="job?.requirements?.length"
              class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6"
            >
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">✅</span>
                <h2 class="text-xl font-semibold text-gray-900">Requisitos</h2>
              </div>
              <ul class="space-y-3">
                <li v-for="(req, idx) in job.requirements" :key="idx" class="flex items-start gap-3 text-gray-600">
                  <span class="text-emerald-600 mt-1">•</span>
                  <span>{{ req }}</span>
                </li>
              </ul>
            </div>

            <!-- Beneficios -->
            <div
              v-if="job?.benefits?.length"
              class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6"
            >
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">🎁</span>
                <h2 class="text-xl font-semibold text-gray-900">Beneficios</h2>
              </div>
              <ul class="space-y-3">
                <li v-for="(benefit, idx) in job.benefits" :key="idx" class="flex items-start gap-3 text-gray-600">
                  <span class="text-emerald-600 mt-1">•</span>
                  <span>{{ benefit }}</span>
                </li>
              </ul>
            </div>

            <!-- Trabajos similares -->
            <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6">
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">🔍</span>
                <h2 class="text-xl font-semibold text-gray-900">Trabajos similares</h2>
              </div>
              <div class="space-y-4">
                <div class="border border-gray-200 rounded-xl p-4 hover:bg-gray-50 transition-colors">
                  <h3 class="font-semibold text-gray-900">Desarrollador Frontend Vue.js</h3>
                  <p class="text-gray-600">DigitalNew - Remoto</p>
                </div>
                <div class="border border-gray-200 rounded-xl p-4 hover:bg-gray-50 transition-colors">
                  <h3 class="font-semibold text-gray-900">UX/UI Designer Junior</h3>
                  <p class="text-gray-600">CreativeStudio - Híbrido</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Barra lateral -->
          <aside v-if="canApply" class="space-y-6 h-fit lg:sticky lg:top-6">
            <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6">
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">🚀</span>
                <h2 class="text-xl font-semibold text-gray-900">Aplicar a esta vacante</h2>
              </div>
              
              <div class="space-y-4 mb-6">
                <div class="flex justify-between items-center">
                  <span class="text-gray-600">Compensación:</span>
                  <span class="text-emerald-600 font-semibold">{{ salaryFormatted }}</span>
                </div>
                <div class="flex justify-between items-center">
                  <span class="text-gray-600">Duración:</span>
                  <span>{{ job?.duration || 'No especificada' }}</span>
                </div>
                <div class="flex justify-between items-center">
                  <span class="text-gray-600">Dedicación:</span>
                  <span>{{ job?.dedication || 'No especificada' }}</span>
                </div>
              </div>

              <div class="space-y-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">Carta de presentación</label>
                  <textarea
                    v-model="coverLetter"
                    class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                    rows="4"
                    placeholder="Habla brevemente de tu interés y experiencia..."
                  ></textarea>
                </div>

                <button
                  @click="handleApply"
                  class="w-full flex items-center justify-center gap-2 bg-emerald-600 hover:bg-emerald-700 text-white px-6 py-3 rounded-xl font-medium transition-colors"
                >
                  <span class="text-lg">📝</span>
                  Aplicar ahora
                </button>

                <div v-if="success" class="bg-emerald-50 text-emerald-700 px-4 py-3 rounded-xl text-sm">
                  {{ success }}
                </div>
                <div v-if="error" class="bg-red-50 text-red-700 px-4 py-3 rounded-xl text-sm">
                  {{ error }}
                </div>

                <p class="text-xs text-gray-400">
                  Al aplicar, aceptas nuestros <a href="#" class="text-emerald-600 hover:underline">términos y condiciones</a>.
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
