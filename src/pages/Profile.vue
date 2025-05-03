<template>
  <DefaultLayout>
    <div class="min-h-screen bg-gradient-to-b from-white to-gray-50 pb-12">
      <!-- Perfil encabezado -->
      <div class="max-w-7xl mx-auto px-4 sm:px-6">
        <div class="relative mb-8">
          <div class="h-48 w-full rounded-2xl bg-gradient-to-r from-emerald-500 to-teal-500 shadow-lg"></div>
          <div class="flex flex-col md:flex-row gap-6 items-start -mt-16 px-4">
            <div class="relative">
              <div class="h-32 w-32 rounded-2xl border-4 border-white bg-emerald-100 flex items-center justify-center text-4xl font-bold text-emerald-600 shadow-lg">
                {{ user.fullName?.charAt(0) || '?' }}
              </div>
              <div class="absolute -bottom-2 -right-2 bg-emerald-600 text-white text-xs font-medium px-2 py-1 rounded-full">
                Nivel 3
              </div>
            </div>
            <div class="pt-16 md:pt-0 flex-1">
              <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div>
                  <div class="flex items-center gap-3 mb-2">
                    <h1 class="text-2xl md:text-3xl font-bold text-gray-900">{{ user.fullName }}</h1>
                    <span class="bg-emerald-100 text-emerald-700 px-3 py-1 rounded-full text-sm font-medium">
                      🎓 Estudiante
                    </span>
                  </div>
                  <p class="text-gray-600">Ingeniería Informática</p>
                </div>
                <div class="flex gap-3">
                  <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
                    <span class="text-lg">📄</span>
                    CV
                  </button>
                  <button class="flex items-center gap-2 border border-gray-200 px-4 py-2 rounded-xl text-sm font-medium hover:bg-gray-50 transition-colors">
                    <span class="text-lg">🔗</span>
                    Compartir
                  </button>
                  <button class="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-colors">
                    <span class="text-lg">✏️</span>
                    Editar perfil
                  </button>
                </div>
              </div>
              <div class="flex flex-wrap gap-4 mt-6">
                <div class="flex items-center gap-2 text-gray-600">
                  <span class="text-lg">🎓</span>
                  <span class="text-sm">{{ user.university || 'Universidad no especificada' }}</span>
                </div>
                <div class="flex items-center gap-2 text-gray-600">
                  <span class="text-lg">📍</span>
                  <span class="text-sm">Buenos Aires, Argentina</span>
                </div>
                <div class="flex items-center gap-2 text-gray-600">
                  <span class="text-lg">📧</span>
                  <span class="text-sm">{{ user.email }}</span>
                </div>
                <div class="flex items-center gap-2 text-gray-600">
                  <span class="text-lg">📅</span>
                  <span class="text-sm">Disponible desde Junio 2023</span>
                </div>
              </div>
              <div v-if="form.skills.length" class="flex flex-wrap gap-2 mt-6">
                <span v-for="skill in form.skills" :key="skill" class="text-sm bg-emerald-50 text-emerald-700 px-3 py-1 rounded-lg border border-emerald-100">
                  {{ skill }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Formulario -->
      <div class="max-w-7xl mx-auto px-4 sm:px-6">
        <div class="bg-white rounded-2xl border border-gray-200 shadow-lg p-8 space-y-8">
          <div class="flex items-center gap-3 mb-6">
            <span class="text-2xl">⚙️</span>
            <h2 class="text-xl font-bold text-gray-900">Configuración del perfil</h2>
          </div>
          <form @submit.prevent="handleSave" class="space-y-6">
            <div class="grid md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Nombre completo</label>
                <input 
                  v-model="form.fullName" 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors" 
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Correo electrónico</label>
                <input 
                  :value="user.email" 
                  disabled 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 bg-gray-50 text-gray-500" 
                />
              </div>
            </div>

            <div v-if="user.role === 'student'" class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Universidad</label>
                <input 
                  v-model="form.university" 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors" 
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Habilidades</label>
                <div class="flex items-center gap-2">
                  <input 
                    v-model="skillsString" 
                    placeholder="Separadas por comas" 
                    class="flex-1 border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors" 
                  />
                  <span class="text-gray-400 text-sm">Ej: JavaScript, Vue, React</span>
                </div>
              </div>
            </div>

            <div v-else-if="user.role === 'employer'" class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Nombre de la empresa</label>
                <input 
                  v-model="form.company.name" 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors" 
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Sitio web</label>
                <input 
                  v-model="form.company.website" 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors" 
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Descripción</label>
                <textarea 
                  v-model="form.company.description" 
                  rows="4" 
                  class="w-full border border-gray-200 rounded-xl px-4 py-3 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-colors"
                ></textarea>
              </div>
            </div>

            <div class="flex justify-end">
              <button 
                type="submit" 
                class="bg-emerald-600 hover:bg-emerald-700 text-white px-6 py-3 rounded-xl font-medium transition-colors"
              >
                Guardar cambios
              </button>
            </div>

            <div v-if="success" class="bg-emerald-50 text-emerald-700 px-4 py-3 rounded-xl text-sm">
              {{ success }}
            </div>
            <div v-if="error" class="bg-red-50 text-red-700 px-4 py-3 rounded-xl text-sm">
              {{ error }}
            </div>
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
