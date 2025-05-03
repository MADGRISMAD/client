<template>
    <DefaultLayout>
      <div class="max-w-7xl mx-auto px-4 py-10">
        <!-- Back -->
        <router-link
          to="/my-jobs"
          class="inline-flex items-center gap-2 text-sm text-gray-600 hover:text-emerald-600 transition-colors mb-6"
        >
          <span>⬅️</span>
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
                <h1 class="text-2xl font-bold text-gray-900">{{ job?.title }}</h1>
                <p class="text-gray-600">{{ job?.company }}</p>
              </div>
              <span class="bg-emerald-100 text-emerald-800 px-3 py-1 text-xs rounded-full font-medium">Activa</span>
            </div>
  
            <div class="flex flex-wrap gap-4 mt-4">
              <div class="flex items-center gap-2 px-3 py-1.5 bg-gray-100 rounded-full text-sm text-gray-600">
                <span class="material-icons text-base">schedule</span>
                {{ job?.duration || 'No especificada' }}
              </div>
              <div class="flex items-center gap-2 px-3 py-1.5 bg-gray-100 rounded-full text-sm text-gray-600">
                <span class="material-icons text-base">event</span>
                Publicada el {{ formatDate(job?.createdAt) }}
              </div>
              <div class="flex items-center gap-2 px-3 py-1.5 bg-gray-100 rounded-full text-sm text-gray-600">
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
            <div class="flex items-center justify-between mb-4">
              <div>
                <h2 class="text-lg font-semibold text-gray-900">Aplicantes</h2>
                <p class="text-sm text-gray-500">{{ applicants.length }} aplicantes en total</p>
              </div>
            </div>
  
            <ul class="space-y-2">
              <li
                v-for="a in applicants"
                :key="a._id"
                @click="selectApplicant(a)"
                class="flex gap-3 p-3 rounded-lg cursor-pointer hover:bg-gray-50 border border-transparent transition-colors"
                :class="{ 'border border-emerald-500 bg-emerald-50/50': selected?.user._id === a.user._id }"
              >
                <div class="h-10 w-10 rounded-full bg-emerald-100 flex items-center justify-center font-semibold text-emerald-600">
                  {{ a.user.fullName.split(' ').map(n => n.charAt(0)).join('').slice(0,2) }}
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex justify-between">
                    <p class="text-sm font-medium text-gray-900 truncate">{{ a.user.fullName }}</p>
                    <span class="text-xs font-medium rounded-full px-2 py-0.5" :class="badgeClass(a.status)">
                      {{ statusLabel(a.status) }}
                    </span>
                  </div>
                  <p class="text-xs text-gray-500 truncate">{{ a.user.university }}</p>
                  <div class="flex flex-wrap gap-1 mt-1">
                    <span
                      v-for="(skill, i) in a.user.skills.slice(0, 3)"
                      :key="i"
                      class="bg-gray-100 text-gray-800 px-2 py-0.5 text-xs rounded-full"
                    >
                      {{ skill }}
                    </span>
                  </div>
                  <p class="text-[10px] text-gray-400 mt-1">Aplicó el {{ formatDate(a.appliedAt) }}</p>
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
                <div class="flex justify-between items-start flex-wrap gap-4">
                  <div>
                    <h2 class="text-lg font-semibold text-gray-900">{{ selected.user.fullName }}</h2>
                    <p class="text-sm text-gray-600">{{ selected.user.degree }}</p>
                  </div>
                  <div class="flex flex-wrap gap-2">
                    <button
                      v-for="status in statuses"
                      :key="status.value"
                      @click="updateStatus(status.value)"
                      class="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium transition-colors"
                      :class="[
                        selected.status === status.value 
                          ? status.activeClass 
                          : status.inactiveClass
                      ]"
                    >
                      <span class="material-icons text-sm">{{ status.icon }}</span>
                      <span>{{ status.label }}</span>
                    </button>
                  </div>
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
              <div class="flex items-center gap-2">
                <span class="material-icons text-gray-400 text-base">email</span>
                <span>{{ selected.user.email }}</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="material-icons text-gray-400 text-base">school</span>
                <span>{{ selected.user.university }}</span>
              </div>
            </div>
  
            <div class="mb-6">
              <h3 class="text-sm font-semibold mb-2 text-gray-900">Carta de presentación</h3>
              <div class="bg-gray-50 rounded-lg p-4">
                <p class="text-sm text-gray-600 whitespace-pre-line">{{ selected.coverLetter }}</p>
              </div>
            </div>
  
            <div>
              <h3 class="text-sm font-semibold mb-3 text-gray-900">Documentos y enlaces</h3>
              <ul class="space-y-2">
                <li v-if="selected.user.cvUrl">
                  <a 
                    :href="selected.user.cvUrl" 
                    target="_blank" 
                    class="flex items-center gap-2 text-emerald-600 hover:text-emerald-700 transition-colors"
                  >
                    <span class="material-icons text-base">description</span>
                    <span>CV - Descargar</span>
                  </a>
                </li>
                <li v-if="selected.user.portfolioUrl">
                  <a 
                    :href="selected.user.portfolioUrl" 
                    target="_blank" 
                    class="flex items-center gap-2 text-emerald-600 hover:text-emerald-700 transition-colors"
                  >
                    <span class="material-icons text-base">language</span>
                    <span>Portafolio</span>
                  </a>
                </li>
                <li v-if="selected.user.githubUrl">
                  <a 
                    :href="selected.user.githubUrl" 
                    target="_blank" 
                    class="flex items-center gap-2 text-emerald-600 hover:text-emerald-700 transition-colors"
                  >
                    <span class="material-icons text-base">code</span>
                    <span>GitHub</span>
                  </a>
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
  
  const statuses = [
    {
      value: 'applied',
      label: 'Aplicado',
      icon: '📨',
      activeClass: 'bg-blue-100 text-blue-800',
      inactiveClass: 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    },
    {
      value: 'viewed',
      label: 'Revisado',
      icon: '👁️',
      activeClass: 'bg-purple-100 text-purple-800',
      inactiveClass: 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    },
    {
      value: 'interview',
      label: 'Entrevista',
      icon: '📅',
      activeClass: 'bg-amber-100 text-amber-800',
      inactiveClass: 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    },
    {
      value: 'hired',
      label: 'Contratado',
      icon: '✅',
      activeClass: 'bg-emerald-100 text-emerald-800',
      inactiveClass: 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    },
    {
      value: 'rejected',
      label: 'Rechazado',
      icon: '❌',
      activeClass: 'bg-red-100 text-red-800',
      inactiveClass: 'bg-gray-100 text-gray-600 hover:bg-gray-200'
    }
  ]
  
  const formatDate = (d) => {
    return new Date(d).toLocaleDateString('es-MX', { year: 'numeric', month: 'long', day: 'numeric' })
  }
  
  const badgeClass = (status) => {
    return statuses.find(s => s.value === status)?.activeClass || ''
  }
  
  const statusLabel = (status) => {
    return statuses.find(s => s.value === status)?.label || ''
  }
  
  const selectApplicant = (a) => {
    selected.value = a
  }
  
  const updateStatus = async (newStatus) => {
    if (!selected.value) return
    const token = localStorage.getItem('token')
    try {
      await axios.put(
        `http://localhost:5000/api/jobs/${route.params.id}/applicants/${selected.value._id}`,
        { status: newStatus },
        { headers: { Authorization: `Bearer ${token}` } }
      )
      selected.value.status = newStatus
    } catch (error) {
      console.error('Error al actualizar el estado:', error)
    }
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
  