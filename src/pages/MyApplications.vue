<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50 py-12">
      <div class="max-w-7xl mx-auto px-4 sm:px-6">
        <!-- Encabezado -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
          <div class="flex items-center gap-3">
            <h1 class="text-2xl md:text-3xl font-bold text-gray-900">Mis postulaciones</h1>
            <span class="bg-emerald-100 text-emerald-700 px-3 py-1 rounded-full text-sm font-medium">
              {{ applications.length }} aplicaciones
            </span>
          </div>
          <button 
            @click="showCalendar = !showCalendar"
            class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors"
          >
            <span class="text-lg">📅</span>
            {{ showCalendar ? 'Ocultar calendario' : 'Ver calendario' }}
          </button>
        </div>

        <!-- Calendario -->
        <div v-if="showCalendar" class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6 mb-8">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-lg font-semibold text-gray-900">Calendario de postulaciones</h2>
            <div class="flex items-center gap-2">
              <span class="text-sm text-gray-600">Leyenda:</span>
              <div class="flex items-center gap-2">
                <span class="w-3 h-3 rounded-full bg-emerald-500"></span>
                <span class="text-sm text-gray-600">Postulación</span>
              </div>
            </div>
          </div>
          <div class="grid grid-cols-7 gap-2">
            <div v-for="day in 31" :key="day" class="aspect-square p-2 border border-gray-100 rounded-lg">
              <div class="text-sm text-gray-600 mb-1">{{ day }}</div>
              <div v-if="hasApplicationOnDay(day)" class="flex justify-center">
                <span class="text-lg">📝</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Sin postulaciones -->
        <div v-if="applications.length === 0" class="bg-white rounded-2xl border border-gray-200 shadow-lg p-8 text-center">
          <div class="text-4xl mb-4">📝</div>
          <h2 class="text-xl font-semibold text-gray-900 mb-2">Aún no has aplicado a ninguna oferta</h2>
          <p class="text-gray-600 mb-6">Explora las ofertas disponibles y comienza tu aventura profesional</p>
          <router-link 
            to="/jobs" 
            class="inline-flex items-center gap-2 bg-emerald-600 hover:bg-emerald-700 text-white px-6 py-3 rounded-xl font-medium transition-colors"
          >
            <span class="text-lg">🔍</span>
            Explorar ofertas
          </router-link>
        </div>

        <!-- Lista de postulaciones -->
        <div v-else class="grid gap-6">
          <div
            v-for="job in applications"
            :key="job._id"
            class="bg-white rounded-2xl border border-gray-200 shadow-lg p-6 hover:shadow-xl transition-all"
          >
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div class="flex-1">
                <div class="flex items-center gap-3 mb-3">
                  <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center text-lg font-semibold text-emerald-600">
                    {{ job.company?.[0] || job.createdBy?.fullName?.[0] || '?' }}
                  </div>
                  <div>
                    <h2 class="text-xl font-semibold text-gray-900">{{ job.title }}</h2>
                    <p class="text-sm text-gray-500">{{ job.company || job.createdBy?.fullName }}</p>
                  </div>
                </div>
                <p class="text-gray-600 mb-4 line-clamp-2">{{ job.description }}</p>
                <div class="flex flex-wrap gap-3">
                  <div class="flex items-center gap-2 text-sm text-gray-600">
                    <span class="text-lg">💰</span>
                    <span>{{ job.salaryRange.min }} - {{ job.salaryRange.max }} {{ job.salaryRange.currency }} / {{ job.salaryRange.type }}</span>
                  </div>
                  <div v-if="job.isRemote" class="flex items-center gap-2 text-sm text-emerald-600">
                    <span class="text-lg">📍</span>
                    <span>Remoto</span>
                  </div>
                  <div class="flex items-center gap-2 text-sm text-gray-600">
                    <span class="text-lg">📅</span>
                    <span>Postulado el {{ formatDate(job.appliedAt) }}</span>
                  </div>
                </div>
              </div>
              <div class="flex flex-col gap-2">
                <div class="flex items-center gap-2">
                  <span class="text-lg">📊</span>
                  <span class="text-sm font-medium" :class="{
                    'text-emerald-600': job.status === 'pending',
                    'text-blue-600': job.status === 'reviewed',
                    'text-gray-600': job.status === 'rejected'
                  }">
                    {{ getStatusText(job.status) }}
                  </span>
                </div>
                <router-link
                  :to="`/jobs/${job._id}`"
                  class="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-colors"
                >
                  <span class="text-lg">👁️</span>
                  Ver detalles
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'
import DefaultLayout from '../layouts/DefaultLayout.vue'

const applications = ref([])
const showCalendar = ref(false)

onMounted(async () => {
  try {
    const token = localStorage.getItem('token')
    const res = await axios.get('http://localhost:5000/api/jobs/my-applications', {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    applications.value = res.data
  } catch (err) {
    console.error('Error al obtener aplicaciones', err)
  }
})

const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('es-ES', { day: 'numeric', month: 'long' })
}

const getStatusText = (status) => {
  const statusMap = {
    pending: 'En revisión',
    reviewed: 'Revisado',
    rejected: 'No seleccionado'
  }
  return statusMap[status] || status
}

const hasApplicationOnDay = (day) => {
  return applications.value.some(app => {
    const appDate = new Date(app.appliedAt)
    return appDate.getDate() === day
  })
}
</script>
  