<template>
  <DefaultLayout>
    <div class="min-h-[100dvh] bg-gradient-to-b from-white to-gray-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 py-12">
        <!-- Encabezado -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
          <router-link 
            to="/jobs" 
            class="flex items-center gap-2 text-sm text-gray-600 hover:text-emerald-600 transition-colors"
          >
            <span class="text-lg"><PhArrowLeft class="ic" /></span>
            Volver a resultados
          </router-link>
          <div class="flex items-center gap-3">
            <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
              <span class="text-lg"><PhStar class="ic" /></span>
              Guardar
            </button>
            <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
              <span class="text-lg"><PhLink class="ic" /></span>
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
                      <PhStar class="ic" /> Destacado
                    </span>
                  </div>
                  <p class="text-gray-600">{{ job?.company || 'Empresa desconocida' }}</p>

                  <div class="flex flex-wrap gap-3 mt-4">
                    <div v-if="job?.isRemote" class="flex items-center gap-2 text-sm text-emerald-600">
                      <span class="text-lg"><PhMapPin class="ic" /></span>
                      <span>Remoto</span>
                    </div>
                    <div class="flex items-center gap-2 text-sm text-gray-600">
                      <span class="text-lg"><PhCalendarBlank class="ic" /></span>
                      <span>Publicado {{ publishedAgo }}</span>
                    </div>
                    <div class="flex items-center gap-2 text-sm text-gray-600">
                      <span class="text-lg"><PhUsers class="ic" /></span>
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
                <span class="text-2xl"><PhNotePencil class="ic" /></span>
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
                <span class="text-2xl"><PhTarget class="ic" /></span>
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
                <span class="text-2xl"><PhCheckCircle class="ic" /></span>
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
                <span class="text-2xl"><PhGift class="ic" /></span>
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
                <span class="text-2xl"><PhMagnifyingGlass class="ic" /></span>
                <h2 class="text-xl font-semibold text-gray-900">Trabajos similares</h2>
              </div>
              <p v-if="similar.length === 0" class="text-sm text-gray-500">Aún no hay otras ofertas publicadas.</p>
              <ul v-else class="space-y-3">
                <li v-for="j in similar" :key="j._id">
                  <router-link :to="`/jobs/${j._id}`" class="block rounded-xl border border-gray-200 p-4 transition-colors hover:bg-gray-50">
                    <h3 class="font-semibold text-gray-900">{{ j.title }}</h3>
                    <p class="text-gray-600">{{ j.company }}<template v-if="j.isRemote"> (remoto)</template></p>
                  </router-link>
                </li>
              </ul>
            </div>
          </div>

          <!-- Barra lateral -->
          <aside v-if="canApply" class="space-y-6 h-fit lg:sticky lg:top-6">
            <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6">
              <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl"><PhRocketLaunch class="ic" /></span>
                <h2 class="text-xl font-semibold text-gray-900">Aplicar a esta vacante</h2>
              </div>
              
              <div v-if="match" class="mb-6 rounded-xl bg-emerald-50 p-4">
                <p class="text-sm font-medium text-gray-900">Encajas <span class="font-mono tabular-nums">{{ match.score }}%</span> con esta vacante</p>
                <p v-if="match.matchedSkills.length" class="mt-1 text-sm text-gray-700">Coinciden: {{ match.matchedSkills.join(', ') }}.</p>
                <p v-if="match.missingSkills.length" class="mt-1 text-sm text-gray-700">Te faltan: {{ match.missingSkills.join(', ') }}.</p>
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
                  <div class="mb-2 flex items-center justify-between gap-3">
                    <label class="block text-sm font-medium text-gray-700">Carta de presentación</label>
                    <button type="button" :disabled="drafting" @click="draftLetter" class="press inline-flex items-center gap-1.5 rounded-lg border border-gray-300 px-3 py-1.5 text-xs font-medium text-gray-800 hover:bg-gray-100 disabled:opacity-60">
                      <PhSparkle class="ic" weight="fill" />
                      {{ drafting ? 'Redactando…' : 'Redactar con IA' }}
                    </button>
                  </div>
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
                  <span class="text-lg"><PhNotePencil class="ic" /></span>
                  Aplicar ahora
                </button>

                <div v-if="success" class="bg-emerald-50 text-emerald-700 px-4 py-3 rounded-xl text-sm">
                  {{ success }}
                </div>
                <div v-if="error" class="bg-red-50 text-red-700 px-4 py-3 rounded-xl text-sm">
                  {{ error }}
                </div>

                <p class="text-xs text-gray-500">Tu carta y tu perfil se envían a la empresa.</p>
              </div>
            </div>
          </aside>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { PhArrowLeft, PhCalendarBlank, PhCheckCircle, PhGift, PhLink, PhMagnifyingGlass, PhMapPin, PhNotePencil, PhRocketLaunch, PhSparkle, PhStar, PhTarget, PhUsers } from '@phosphor-icons/vue'
import { API_URL } from '../config'
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuth } from '../composables/useAuth'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import AiService from '../services/AiService'
import axios from 'axios'

const route = useRoute()
const { user, isLoggedIn } = useAuth()

const job = ref(null)
const coverLetter = ref('')
const success = ref('')
const error = ref('')
const similar = ref([])
const match = ref(null)
const drafting = ref(false)

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
    return
  }
  // otras ofertas reales (no la actual)
  JobService.getAll({ limit: 4 })
    .then(list => (similar.value = list.filter(j => j._id !== job.value._id).slice(0, 3)))
    .catch(() => {})
  // qué tanto encaja el estudiante (si la IA o el perfil no están disponibles, simplemente no se muestra)
  if (canApply.value) AiService.match(job.value._id).then(m => (match.value = m)).catch(() => {})
})

const draftLetter = async () => {
  error.value = ''
  drafting.value = true
  try {
    coverLetter.value = await AiService.coverLetter(job.value._id)
  } catch (err) {
    error.value = err.response?.data?.message || 'No se pudo redactar la carta. Escríbela tú o inténtalo de nuevo.'
  } finally {
    drafting.value = false
  }
}

const handleApply = async () => {
  error.value = ''
  success.value = ''
  try {
    const token = localStorage.getItem('token')
    await axios.post(
      `${API_URL}/api/jobs/${job.value._id}/apply`,
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
