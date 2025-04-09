<template>
  <DefaultLayout>
    <div class="max-w-5xl mx-auto py-10 px-4">
      <h1 class="text-2xl font-bold mb-6">Mis vacantes publicadas</h1>

      <div v-if="jobs.length === 0" class="text-gray-500">Aún no has publicado vacantes.</div>

      <div v-else class="grid gap-4">
        <div v-for="job in jobs" :key="job._id" class="border p-4 rounded shadow-sm bg-white">
          <h2 class="text-lg font-semibold">{{ job.title }}</h2>
          <p class="text-sm text-gray-600 mb-2">{{ job.description.slice(0, 100) }}...</p>
          <div class="flex items-center gap-2 text-sm text-gray-500">
            <span>{{ job.salaryRange.min }} - {{ job.salaryRange.max }} {{ job.salaryRange.currency }} / {{ job.salaryRange.type }}</span>
            <span v-if="job.isRemote">• Remoto</span>
            <span v-if="job.highlighted">• Destacado</span>
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

const jobs = ref([])

onMounted(async () => {
  const token = localStorage.getItem('token')
  const res = await axios.get('http://localhost:5000/api/jobs/my-jobs', {
    headers: { Authorization: `Bearer ${token}` }
  })
  jobs.value = res.data
})
</script>
