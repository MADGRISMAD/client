<template>
    <DefaultLayout>
      <div class="max-w-6xl mx-auto px-4 py-10">
        <div class="flex items-center justify-between mb-6">
          <h1 class="text-2xl font-bold">Mis vacantes publicadas</h1>
          <router-link
            to="/jobs/create"
            class="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 text-sm"
          >
            + Nueva vacante
          </router-link>
        </div>
  
        <div v-if="jobs.length === 0" class="text-gray-500">
          No has publicado vacantes aún.
        </div>
  
        <div v-else class="grid gap-6 md:grid-cols-2">
          <div
            v-for="job in jobs"
            :key="job._id"
            class="border border-gray-200 p-5 rounded-lg shadow-sm bg-white hover:shadow-md transition"
          >
            <div class="flex justify-between items-start mb-2">
              <h2 class="text-lg font-semibold text-gray-800">{{ job.title }}</h2>
              <span v-if="job.highlighted" class="text-xs text-green-600 bg-green-100 px-2 py-1 rounded-full">
                Destacado
              </span>
            </div>
            <p class="text-sm text-gray-500 mb-2">{{ job.company }}</p>
            <p class="text-sm text-gray-600 line-clamp-2 mb-4">{{ job.description }}</p>
  
            <div class="flex justify-between items-center text-sm">
              <p class="text-gray-500">Aplicantes: {{ job.applicants?.length || 0 }}</p>
              <router-link
                :to="`/jobs/${job._id}`"
                class="text-green-600 hover:underline text-sm"
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
  import { onMounted, ref } from 'vue'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  import JobService from '../services/JobService'
  import { useAuth } from '../composables/useAuth'
  
  const { user } = useAuth()
  const jobs = ref([])
  const error = ref('')
  
  onMounted(async () => {
    try {
      const all = await JobService.getAll()
      jobs.value = all.filter(job => job.createdBy === user.value._id)
    } catch (err) {
      error.value = 'No se pudieron cargar tus vacantes'
      console.error(err)
    }
  })
  </script>
  