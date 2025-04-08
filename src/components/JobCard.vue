<template>
    <div class="border rounded shadow-sm p-4 bg-white hover:shadow-md transition">
      <h3 class="text-lg font-semibold text-green-700">{{ job.title }}</h3>
      <p class="text-sm text-gray-600 mb-2">{{ job.companyName }}</p>
      <span class="text-xs inline-block bg-gray-100 px-2 py-1 rounded text-gray-600">
        {{ job.isRemote ? 'Remoto' : 'Presencial' }}
      </span>
  
      <router-link
        :to="`/jobs/${job._id}`"
        class="mt-3 inline-block text-sm text-green-700 hover:underline"
      >
        Ver detalles →
      </router-link>
    </div>
  </template>
  
  <script setup>
  defineProps({ job: Object })
  </script>
  