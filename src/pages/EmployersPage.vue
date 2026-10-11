<template>
  <DefaultLayout>
    <div class="bg-white">
      <!-- Portada: mensaje a la izquierda, foto a la derecha -->
      <section class="mx-auto grid max-w-7xl items-center gap-12 px-4 pb-20 pt-14 sm:px-6 lg:grid-cols-[1.1fr_0.9fr] lg:gap-16 lg:pt-20">
        <div>
          <h1 class="rise text-4xl font-semibold leading-[1.05] tracking-tighter text-gray-950 sm:text-5xl lg:text-6xl" style="--i: 0">Encuentra talento universitario</h1>
          <p class="rise mt-6 max-w-[48ch] text-lg leading-relaxed text-gray-600" style="--i: 1">
            Publica tu vacante en minutos y recibe postulaciones de estudiantes con correo universitario.
          </p>
          <div class="rise mt-9 flex flex-wrap items-center gap-3" style="--i: 2">
            <router-link :to="primary.to" class="press inline-flex items-center gap-2 whitespace-nowrap rounded-lg bg-emerald-600 px-6 py-3 text-sm font-medium text-white hover:bg-emerald-700">
              {{ primary.label }}
              <PhArrowRight :size="16" />
            </router-link>
          </div>
        </div>
        <div class="rise aspect-[4/3] overflow-hidden rounded-xl bg-gray-100 shadow-lg" style="--i: 2">
          <img src="/img/oficio.jpg" width="1200" height="900" alt="Persona sirviendo café con una tetera en un mostrador" fetchpriority="high" class="settle h-full w-full object-cover" />
        </div>
      </section>

      <!-- Qué incluye: lista en dos columnas con icono, sin tarjetas -->
      <section class="border-y border-gray-200 bg-gray-50">
        <div class="mx-auto max-w-7xl px-4 py-24 sm:px-6 md:py-28">
          <h2 v-reveal class="max-w-xl text-3xl font-semibold leading-tight tracking-tight text-gray-950 md:text-4xl">Lo que ya puedes hacer hoy</h2>
          <dl class="mt-14 grid gap-x-16 gap-y-12 md:grid-cols-2">
            <div v-for="(f, i) in features" :key="f.title" v-reveal="i" class="flex gap-5">
              <component :is="f.icon" :size="28" class="mt-0.5 shrink-0 text-emerald-700" />
              <div>
                <dt class="text-lg font-semibold tracking-tight text-gray-950">{{ f.title }}</dt>
                <dd class="mt-2 max-w-[44ch] leading-relaxed text-gray-600">{{ f.text }}</dd>
              </div>
            </div>
          </dl>
        </div>
      </section>

      <!-- Cierre -->
      <section class="mx-auto max-w-7xl px-4 py-24 sm:px-6 md:py-28">
        <div v-reveal class="flex flex-col items-start justify-between gap-8 rounded-xl bg-emerald-700 p-8 text-white md:flex-row md:items-center md:p-14">
          <h2 class="max-w-xl text-3xl font-semibold leading-tight tracking-tight md:text-4xl">Tu primera vacante, hoy</h2>
          <router-link :to="primary.to" class="press inline-flex items-center justify-center whitespace-nowrap rounded-lg bg-white px-6 py-3 text-sm font-medium text-emerald-800 hover:bg-emerald-50">
            {{ primary.label }}
          </router-link>
        </div>
      </section>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { computed } from 'vue'
import { PhArrowRight, PhPencilLine, PhUsers, PhBell, PhGraduationCap } from '@phosphor-icons/vue'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import { useAuth } from '../composables/useAuth'

const { isLoggedIn, user } = useAuth()

// Una sola acción, con el mismo nombre en toda la página
const primary = computed(() => {
  if (isLoggedIn() && user.value?.role === 'employer') return { to: '/jobs/create', label: 'Publicar vacante' }
  if (isLoggedIn()) return { to: '/jobs', label: 'Ver ofertas' }
  return { to: '/register', label: 'Crear cuenta' }
})

const features = [
  { title: 'Vacantes mejor escritas', text: 'La IA pule la descripción de tu puesto y sugiere etiquetas, requisitos y beneficios. Tú decides qué publicar.', icon: PhPencilLine },
  { title: 'Estudiantes con correo universitario', text: 'Cada postulante se registra con su correo de la universidad, así sabes que estudia.', icon: PhGraduationCap },
  { title: 'Todos los postulantes en un lugar', text: 'Lee cada carta y perfil, y cambia el estado: revisado, entrevista, contratado o rechazado.', icon: PhUsers },
  { title: 'Avisos en cada paso', text: 'Te avisamos cuando alguien se postula, y el estudiante recibe un aviso cuando cambias su estado.', icon: PhBell }
]
</script>
