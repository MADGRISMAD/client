<template>
    <DefaultLayout>
      <div class="max-w-7xl mx-auto px-4 py-10">
        <!-- Back -->
        <router-link
          to="/my-jobs"
          class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-emerald-600 mb-6"
        >
          <span class="material-icons text-base">arrow_back</span>
          Volver a mis vacantes
        </router-link>
  
        <!-- Header -->
        <div class="flex items-start gap-6 mb-10">
          <div class="h-16 w-16 bg-emerald-100 rounded-lg flex items-center justify-center text-2xl font-bold text-emerald-600">
            {{ job?.title?.charAt(0) || 'T' }}
          </div>
          <div class="flex-1">
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div>
                <h1 class="text-2xl font-bold">{{ job?.title }}</h1>
                <p class="text-muted-foreground">{{ job?.company }}</p>
              </div>
              <span class="bg-emerald-100 text-emerald-800 px-3 py-1 text-xs rounded-full">Activa</span>
            </div>
  
            <div class="flex flex-wrap gap-4 mt-4 text-sm text-muted-foreground">
              <div class="flex items-center gap-1">
                <span class="material-icons text-base">schedule</span>
                {{ job?.duration || 'No especificada' }}
              </div>
              <div class="flex items-center gap-1">
                <span class="material-icons text-base">event</span>
                Publicada el {{ formatDate(job?.createdAt) }}
              </div>
              <div class="flex items-center gap-1">
                <span class="material-icons text-base">group</span>
                {{ applicants.length }} aplicantes
              </div>
            </div>
          </div>
        </div>
  
        <!-- Main Content -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Sidebar: Lista de aplicantes -->
          <div class="bg-white rounded-lg border border-gray-200 shadow-sm p-4">
            <h2 class="text-lg font-semibold mb-2">Aplicantes</h2>
            <p class="text-sm text-muted-foreground mb-4">{{ applicants.length }} aplicantes en total</p>
  
            <ul class="space-y-2">
              <li
                v-for="a in applicants"
                :key="a._id"
                @click="selectApplicant(a)"
                class="flex gap-3 p-3 rounded-lg cursor-pointer hover:bg-gray-50 border border-transparent"
                :class="{ 'border border-emerald-500 bg-gray-50': selected?.user._id === a.user._id }"
              >
                <div class="h-10 w-10 rounded-full bg-emerald-100 flex items-center justify-center font-semibold text-emerald-600">
                  {{ a.user.fullName.split(' ').map(n => n.charAt(0)).join('').slice(0,2) }}
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex justify-between">
                    <p class="text-sm font-medium truncate">{{ a.user.fullName }}</p>
                    <span class="text-xs font-medium rounded-full px-2 py-0.5" :class="badgeClass(a.status)">
                      {{ statusLabel(a.status) }}
                    </span>
                  </div>
                  <p class="text-xs text-muted-foreground truncate">{{ a.user.university }}</p>
                  <div class="flex flex-wrap gap-1 mt-1">
                    <span
                      v-for="(skill, i) in a.user.skills.slice(0, 3)"
                      :key="i"
                      class="bg-gray-100 text-gray-800 px-2 py-0.5 text-xs rounded-full"
                    >
                      {{ skill }}
                    </span>
                  </div>
                  <p class="text-[10px] text-muted-foreground mt-1">Aplicó el {{ formatDate(a.appliedAt) }}</p>
                </div>
              </li>
            </ul>
          </div>
  
          <!-- Detalle del aplicante -->
          <div class="lg:col-span-2 bg-white rounded-lg border border-gray-200 shadow-sm p-6" v-if="selected">
            <div class="flex items-start gap-4 mb-6">
              <div class="h-14 w-14 rounded-full bg-emerald-100 flex items-center justify-center font-bold text-emerald-600 text-xl">
                {{ selected.user.fullName.split(' ').map(n => n.charAt(0)).join('').slice(0,2) }}
              </div>
              <div class="flex-1">
                <div class="flex justify-between items-start flex-wrap">
                  <div>
                    <h2 class="text-lg font-semibold">{{ selected.user.fullName }}</h2>
                    <p class="text-sm text-muted-foreground">{{ selected.user.degree }}</p>
                  </div>
                  <select
                    v-model="selected.status"
                    @change="updateStatus(selected.status)"
                    class="border border-gray-300 px-2 py-1 rounded-md text-sm"
                  >
                    <option value="applied">Aplicado</option>
                    <option value="viewed">Revisado</option>
                    <option value="interview">Entrevista</option>
                    <option value="hired">Contratado</option>
                    <option value="rejected">Rechazado</option>
                  </select>
                </div>
  
                <div class="flex flex-wrap gap-2 mt-2">
                  <span
                    v-for="(skill, i) in selected.user.skills"
                    :key="i"
                    class="bg-gray-100 text-gray-800 px-2 py-1 text-xs rounded-full"
                  >
                    {{ skill }}
                  </span>
                </div>
              </div>
            </div>
  
            <div class="grid md:grid-cols-2 gap-4 text-sm text-gray-700 mb-6">
              <div><span class="font-medium">Email:</span> {{ selected.user.email }}</div>
              <div><span class="font-medium">Universidad:</span> {{ selected.user.university }}</div>
            </div>
  
            <div class="mb-4">
              <h3 class="text-sm font-semibold mb-1 text-gray-700">Carta de presentación</h3>
              <p class="text-sm text-gray-600 whitespace-pre-line">{{ selected.coverLetter }}</p>
            </div>
  
            <div>
              <h3 class="text-sm font-semibold mb-2 text-gray-700">Documentos y enlaces</h3>
              <ul class="space-y-2 text-sm">
                <li v-if="selected.user.cvUrl">
                  <a :href="selected.user.cvUrl" target="_blank" class="text-emerald-600 hover:underline">📄 CV - Descargar</a>
                </li>
                <li v-if="selected.user.portfolioUrl">
                  <a :href="selected.user.portfolioUrl" target="_blank" class="text-emerald-600 hover:underline">🌐 Portafolio</a>
                </li>
                <li v-if="selected.user.githubUrl">
                  <a :href="selected.user.githubUrl" target="_blank" class="text-emerald-600 hover:underline">💻 GitHub</a>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </DefaultLayout>
  </template>
  
  
  <script setup>
  import { ref, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import axios from 'axios'
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  
  const route = useRoute()
  const applicants = ref([])
  const selected = ref(null)
  const job = ref(null)
  
  const formatDate = (d) => {
    return new Date(d).toLocaleDateString('es-MX', { year: 'numeric', month: 'long', day: 'numeric' })
  }
  
  const badgeClass = (status) => ({
    applied: 'bg-blue-100 text-blue-800',
    viewed: 'bg-purple-100 text-purple-800',
    interview: 'bg-amber-100 text-amber-800',
    hired: 'bg-emerald-100 text-emerald-800',
    rejected: 'bg-red-100 text-red-800'
  }[status])
  
  const statusLabel = (status) => ({
    applied: 'Aplicado',
    viewed: 'Revisado',
    interview: 'Entrevista',
    hired: 'Contratado',
    rejected: 'Rechazado'
  }[status])
  
  const selectApplicant = (a) => {
    selected.value = a
  }
  
  const updateStatus = async (newStatus) => {
    if (!selected.value) return
    const token = localStorage.getItem('token')
    await axios.put(
      `http://localhost:5000/api/jobs/${route.params.id}/applicants/${selected.value._id}`,
      { status: newStatus },
      { headers: { Authorization: `Bearer ${token}` } }
    )
  }
  
  onMounted(async () => {
    const token = localStorage.getItem('token')
    const [applicantsRes, jobRes] = await Promise.all([
      axios.get(`http://localhost:5000/api/jobs/${route.params.id}/applicants`, {
        headers: { Authorization: `Bearer ${token}` },
      }),
      axios.get(`http://localhost:5000/api/jobs/${route.params.id}`, {
        headers: { Authorization: `Bearer ${token}` },
      }),
    ])
    applicants.value = applicantsRes.data
    job.value = jobRes.data
  })
  </script>
  