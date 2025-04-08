<template>
    <DefaultLayout>
      <div class="max-w-7xl mx-auto py-6 px-4">
        <!-- Barra superior -->
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-2 text-sm text-gray-500">
            <!-- Volver a resultados -->
            <router-link to="/jobs" class="flex items-center gap-1 hover:underline">
              <span class="material-icons text-base">arrow_back</span>
              Volver a resultados
            </router-link>
          </div>
  
          <!-- Acciones (Guardar, Compartir) -->
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
  
        <!-- Contenedor principal (2 columnas) -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Columna izquierda: Contenido principal -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Encabezado de la vacante -->
            <section class="bg-white border rounded-md p-6 shadow-sm">
              <h1 class="text-2xl font-bold mb-1">{{ job?.title }}</h1>
              <p class="text-gray-600 text-sm mb-2">{{ job?.company }}</p>
              
              <div class="flex flex-wrap gap-2 items-center text-sm text-gray-500 mb-3">
                <!-- Ubicación/Remoto -->
                <span v-if="job?.isRemote" class="bg-green-50 text-green-600 px-2 py-1 rounded-full text-xs">
                  Remoto
                </span>
                <!-- Etiquetas / Ubicación -->
                <span v-for="tag in job?.tags" :key="tag" class="bg-gray-100 text-gray-600 px-2 py-1 rounded-full text-xs">
                  {{ tag }}
                </span>
                <!-- Número de aplicantes -->
                <span>
                  • {{ job?.applicants?.length || 0 }} aplicantes
                </span>
              </div>
  
              <div class="text-sm text-gray-400">
                Publicado el {{ formatDate(job?.createdAt) }}
              </div>
            </section>
  
            <!-- Tabs de contenido (si quieres tabs, puedes usarlas; aquí va en secciones directas) -->
            <!-- Descripción -->
            <section class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-3">Descripción del puesto</h2>
              <div class="text-gray-700 whitespace-pre-line">
                {{ job?.description }}
              </div>
            </section>
  
            <!-- Responsabilidades -->
            <section v-if="job?.responsibilities?.length" class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-3">Responsabilidades</h2>
              <ul class="list-disc ml-5 space-y-1 text-gray-700">
                <li v-for="(resp, idx) in job.responsibilities" :key="idx">{{ resp }}</li>
              </ul>
            </section>
  
            <!-- Requisitos -->
            <section v-if="job?.requirements?.length" class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-3">Requisitos</h2>
              <ul class="list-disc ml-5 space-y-1 text-gray-700">
                <li v-for="(req, idx) in job.requirements" :key="idx">{{ req }}</li>
              </ul>
            </section>
  
            <!-- Beneficios -->
            <section v-if="job?.benefits?.length" class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-3">Beneficios</h2>
              <ul class="list-disc ml-5 space-y-1 text-gray-700">
                <li v-for="(benefit, idx) in job.benefits" :key="idx">{{ benefit }}</li>
              </ul>
            </section>
  
            <!-- Trabajos similares (Placeholder) -->
            <section class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-3">Trabajos similares</h2>
              <div class="space-y-4 text-sm">
                <div class="border rounded-md p-4 hover:shadow-sm transition">
                  <h3 class="font-semibold">Desarrollador Frontend Vue.js</h3>
                  <p class="text-gray-500">DigitalNew - Remoto</p>
                </div>
                <div class="border rounded-md p-4 hover:shadow-sm transition">
                  <h3 class="font-semibold">UX/UI Designer Junior</h3>
                  <p class="text-gray-500">CreativeStudio - Híbrido</p>
                </div>
              </div>
            </section>
          </div>
  
          <!-- Columna derecha: Card de aplicación -->
          <aside v-if="canApply" class="space-y-4 h-fit lg:sticky lg:top-6">

            <div class="bg-white border rounded-md p-6 shadow-sm">
              <h2 class="text-lg font-semibold mb-4">Aplicar a esta vacante</h2>
  
              <div class="text-sm space-y-2 mb-6 text-gray-700">
                <div>
                  <strong>Compensación:</strong>
                  <span>{{ salaryFormatted }}</span>
                </div>
                <div>
                  <strong>Duración:</strong>
                  <span>{{ job?.duration || 'No especificada' }}</span>
                </div>
                <div>
                  <strong>Dedicación:</strong>
                  <span>{{ job?.dedication || 'No especificada' }}</span>
                </div>
              </div>
  
              <div >
                <label class="block text-sm mb-1 font-medium">Carta de presentación</label>
                <textarea
                  v-model="coverLetter"
                  class="w-full border rounded px-3 py-2 text-sm mb-3"
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
  import { useRoute, useRouter } from 'vue-router'
  import { useAuth } from '../composables/useAuth'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  import JobService from '../services/JobService'
  import axios from 'axios'
  
  const route = useRoute()
  const router = useRouter()
  const { user, isLoggedIn } = useAuth()
  
  const job = ref(null)
  const coverLetter = ref('')
  const success = ref('')
  const error = ref('')
  const canApply = computed(() => {
  return (
    isLoggedIn() &&
    user.value?.role === 'student' &&
    job.value?.createdBy !== user.value._id
  )
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
  
  // Formatear fecha
  const formatDate = (dateString) => {
    const date = new Date(dateString)
    return date.toLocaleDateString('es-MX', { year: 'numeric', month: 'long', day: 'numeric' })
  }
  
  // Formatear salario
  const salaryFormatted = computed(() => {
    if (!job.value?.salaryRange) return 'No especificado'
    const s = job.value.salaryRange
    return `$${s.min}-${s.max}/${s.type} ${s.currency}`
  })
  </script>
  