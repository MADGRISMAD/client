<template>
    <DefaultLayout>
      <div class="max-w-6xl mx-auto mt-10 px-4">
        <h1 class="text-2xl font-bold mb-6">Ofertas disponibles</h1>
  
        <div v-if="jobs.length === 0" class="text-gray-500">
          No hay ofertas disponibles por ahora.
        </div>
  
        <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="job in jobs"
            :key="job._id"
            class="border rounded-lg p-4 shadow-sm hover:shadow-md transition"
          >
            <h2 class="text-lg font-semibold text-green-700">{{ job.title }}</h2>
            <p class="text-sm text-gray-500 mb-2">{{ job.companyName || job.createdBy?.fullName }}</p>
            <p class="text-sm text-gray-600 mb-4">{{ job.description.slice(0, 100) }}...</p>
            <router-link :to="`/jobs/${job._id}`" class="text-green-600 hover:underline text-sm">Ver más</router-link>
          </div>
        </div>
      </div>
    </DefaultLayout>
  </template>
  
  <script setup>
  import { onMounted, ref } from 'vue'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  import JobService from '../services/JobService'
  
  const jobs = ref([])
  
  onMounted(async () => {
    try {
      jobs.value = await JobService.getAll()
    } catch (err) {
      console.error('Error al cargar ofertas:', err)
    }
  })
  </script>
  