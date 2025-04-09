<template>
  <DefaultLayout>
    <section class="min-h-screen flex items-center justify-center bg-gradient-to-b from-white to-gray-50 py-16 px-4">
      <div class="w-full max-w-2xl bg-white border border-gray-200 rounded-2xl shadow-md p-10 space-y-6">
        <div class="text-center">
          <h2 class="text-3xl font-bold text-gray-900">Crear cuenta en internships.gg</h2>
          <p class="text-sm text-gray-500 mt-1">Únete como estudiante o empresa</p>
        </div>

        <form @submit.prevent="handleRegister" class="space-y-5">
          <!-- Rol -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Tipo de cuenta</label>
            <select
              v-model="form.role"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
              required
            >
              <option value="student">Estudiante</option>
              <option value="employer">Empresa</option>
            </select>
          </div>

          <!-- Nombre -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Nombre completo</label>
            <input
              v-model="form.fullName"
              type="text"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
              placeholder="Tu nombre completo"
              required
            />
          </div>

          <!-- Correo -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Correo electrónico</label>
            <input
              v-model="form.email"
              type="email"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
              placeholder="nombre@universidad.edu"
              required
            />
            <p
              v-if="form.role === 'student' && !emailEndsInEdu"
              class="text-sm text-red-600 mt-1"
            >
              El correo debe terminar en <strong>.edu</strong>
            </p>
          </div>

          <!-- Contraseña -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Contraseña</label>
            <input
              v-model="form.password"
              type="password"
              class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
              placeholder="••••••••"
              required
            />
          </div>

          <!-- Campos estudiante -->
          <div v-if="form.role === 'student'" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Universidad</label>
              <input
                v-model="form.university"
                type="text"
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
                placeholder="Nombre de tu universidad"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Habilidades (separadas por coma)</label>
              <input
                v-model="skillsString"
                type="text"
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
                placeholder="Ej. Vue.js, Python, Excel"
              />
            </div>
          </div>

          <!-- Campos empresa -->
          <div v-if="form.role === 'employer'" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Nombre de la empresa</label>
              <input
                v-model="form.company.name"
                type="text"
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
                placeholder="Ej. TalentCorp"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Sitio web</label>
              <input
                v-model="form.company.website"
                type="url"
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
                placeholder="https://"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Descripción</label>
              <textarea
                v-model="form.company.description"
                rows="3"
                class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:ring-emerald-500 focus:outline-none"
                placeholder="Breve descripción de tu empresa"
              ></textarea>
            </div>
          </div>

          <!-- Botón -->
          <button
            type="submit"
            class="w-full bg-emerald-600 hover:bg-emerald-700 text-white font-semibold py-3 rounded-lg transition"
          >
            Registrarse
          </button>

          <p v-if="error" class="text-sm text-red-600 text-center">{{ error }}</p>
          <p v-if="success" class="text-sm text-emerald-600 text-center">{{ success }}</p>
        </form>
      </div>
    </section>
  </DefaultLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import AuthService from '../services/AuthService'
import DefaultLayout from '../layouts/DefaultLayout.vue'

const router = useRouter()
const error = ref('')
const success = ref('')
const skillsString = ref('')

const form = ref({
  role: 'student',
  fullName: '',
  email: '',
  password: '',
  university: '',
  skills: [],
  company: {
    name: '',
    website: '',
    description: ''
  }
})

const emailEndsInEdu = computed(() => form.value.email.trim().endsWith('.edu'))

const handleRegister = async () => {
  error.value = ''
  success.value = ''

  if (form.value.role === 'student' && !emailEndsInEdu.value) {
    error.value = 'El correo debe terminar en .edu'
    return
  }

  if (form.value.role === 'student') {
    form.value.skills = skillsString.value
      .split(',')
      .map(s => s.trim())
      .filter(Boolean)
  }

  try {
    const res = await AuthService.register(form.value)
    success.value = res.message || 'Registro exitoso. Revisa tu correo.'
    setTimeout(() => {
      router.push('/login')
    }, 1500)
  } catch (err) {
    error.value = err.response?.data?.message || 'Error al registrar'
  }
}
</script>
