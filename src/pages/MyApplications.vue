<template>
    <DefaultLayout>
      <div class="max-w-5xl mx-auto py-10 px-4">
        <div class="flex items-center justify-between mb-6">
          <h1 class="text-2xl font-bold">Mis aplicaciones</h1>
        </div>
  
        <div v-if="applications.length === 0" class="text-gray-500">
          Aún no has aplicado a ninguna oferta.
        </div>
  
        <div v-else class="grid gap-4">
          <div
            v-for="job in applications"
            :key="job._id"
            class="border p-4 rounded shadow-sm bg-white hover:shadow transition"
          >
            <div class="flex justify-between items-center">
              <div>
                <h2 class="text-lg font-semibold">{{ job.title }}</h2>
                <p class="text-sm text-gray-600 mb-2">
                  {{ job.description.slice(0, 100) }}...
                </p>
                <div class="flex items-center gap-2 text-sm text-gray-500">
                  <span>
                    {{ job.salaryRange.min }} - {{ job.salaryRange.max }}
                    {{ job.salaryRange.currency }} / {{ job.salaryRange.type }}
                  </span>
                  <span v-if="job.isRemote">• Remoto</span>
                </div>
              </div>
              <RouterLink
                :to="`/jobs/${job._id}`"
                class="text-sm text-emerald-600 font-medium hover:underline"
              >
                Ver detalles
              </RouterLink>
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
  </script>
  