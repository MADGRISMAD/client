<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50 pb-12">
      

      <!-- Perfil encabezado -->
      <div class=" container mx-auto"> 
        <div class="relative mb-8">
          <div class="h-48 w-full rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500"></div>
          <div class="flex flex-col md:flex-row gap-6 items-start -mt-16 px-4">
            <div class=" h-32 w-32 rounded-full border-4 border-white bg-emerald-100 flex items-center justify-center text-4xl font-bold text-emerald-600 shadow-sm">
              {{ user.fullName?.charAt(0) || '?' }}
            </div>
            <div class="pt-16 md:pt-0 flex-1">
              <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div>
                  <h1 class="text-2xl font-bold">{{ user.fullName }}</h1>
                  <p class="text-gray-500">Estudiante de Ingeniería Informática</p>
                </div>
                <div class="flex gap-3">
                  <button class="border px-3 py-1 rounded text-sm hover:bg-gray-50">📄 CV</button>
                  <button class="border px-3 py-1 rounded text-sm hover:bg-gray-50">🔗 Compartir</button>
                  <button class="bg-emerald-600 hover:bg-emerald-700 text-white px-3 py-1 rounded text-sm">✏️ Editar perfil</button>
                </div>
              </div>
              <div class="flex flex-wrap gap-4 mt-4 text-sm text-gray-600">
                <div>🎓 {{ user.university || 'Universidad no especificada' }}</div>
                <div>📍 Buenos Aires, Argentina</div>
                <div>📧 {{ user.email }}</div>
                <div>📅 Disponible desde Junio 2023</div>
              </div>
              <div v-if="form.skills.length" class="flex flex-wrap gap-2 mt-4">
                <span v-for="skill in form.skills" :key="skill" class="text-xs bg-emerald-100 text-emerald-800 px-2 py-1 rounded-full">
                  {{ skill }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Formulario -->
      <div class="max-w-5xl mx-auto mt-8 px-6">
        <div class="bg-white border shadow rounded-xl p-8 space-y-6">
          <form @submit.prevent="handleSave" class="space-y-6 text-gray-700">
            <div>
              <label class="block text-sm font-medium mb-1">Nombre completo</label>
              <input v-model="form.fullName" class="w-full border rounded-md px-3 py-2" />
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Correo electrónico</label>
              <input :value="user.email" disabled class="w-full border rounded-md px-3 py-2 bg-gray-100 text-gray-500" />
            </div>
            <div v-if="user.role === 'student'">
              <label class="block text-sm font-medium mb-1">Universidad</label>
              <input v-model="form.university" class="w-full border rounded-md px-3 py-2" />
              <label class="block text-sm font-medium mt-4 mb-1">Habilidades (separadas por coma)</label>
              <input v-model="skillsString" class="w-full border rounded-md px-3 py-2" />
            </div>
            <div v-else-if="user.role === 'employer'">
              <label class="block text-sm font-medium mb-1">Nombre de la empresa</label>
              <input v-model="form.company.name" class="w-full border rounded-md px-3 py-2" />
              <label class="block text-sm font-medium mt-4 mb-1">Sitio web</label>
              <input v-model="form.company.website" class="w-full border rounded-md px-3 py-2" />
              <label class="block text-sm font-medium mt-4 mb-1">Descripción</label>
              <textarea v-model="form.company.description" class="w-full border rounded-md px-3 py-2" rows="3" />
            </div>
            <div>
              <button type="submit" class="bg-emerald-600 hover:bg-emerald-700 text-white px-6 py-2 rounded-md font-medium">
                Guardar cambios
              </button>
            </div>
            <p v-if="success" class="text-sm text-green-600">{{ success }}</p>
            <p v-if="error" class="text-sm text-red-500">{{ error }}</p>
          </form>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, reactive, watch } from 'vue'
import { useAuth } from '../composables/useAuth'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import UserService from '../services/UserService'

const { user } = useAuth()

const form = reactive({
  fullName: user.value.fullName,
  university: user.value.university || '',
  skills: user.value.skills || [],
  company: {
    name: user.value.company?.name || '',
    website: user.value.company?.website || '',
    description: user.value.company?.description || ''
  }
})

const skillsString = ref(form.skills.join(', '))

watch(skillsString, val => {
  form.skills = val.split(',').map(s => s.trim()).filter(Boolean)
})

const success = ref('')
const error = ref('')

const handleSave = async () => {
  error.value = ''
  success.value = ''
  try {
    await UserService.updateProfile(form)
    success.value = 'Perfil actualizado correctamente.'
  } catch (err) {
    error.value = err.response?.data?.message || 'Error al actualizar perfil.'
  }
}
</script>
