<template>
  <DefaultLayout>
    <div class="bg-white">
      <!-- Portada: texto y búsqueda a la izquierda, foto a la derecha (en celular, una sola columna) -->
      <section class="mx-auto grid min-h-[calc(100dvh-4rem)] max-w-7xl items-center gap-12 px-4 pb-16 pt-14 sm:px-6 lg:grid-cols-[1.15fr_0.85fr] lg:gap-16 lg:pt-20">
        <div>
          <h1 class="rise text-4xl font-semibold leading-[1.05] tracking-tighter text-gray-950 sm:text-5xl lg:text-6xl" style="--i: 0">
            Encuentra tu primer empleo con IA
          </h1>
          <p class="rise mt-6 max-w-[48ch] text-lg leading-relaxed text-gray-600" style="--i: 1">
            Describe el trabajo que quieres con tus palabras. IAplica busca por significado y te dice qué tanto encajas.
          </p>

          <form class="rise mt-9 max-w-xl" style="--i: 2" role="search" @submit.prevent="goSearch()">
            <label for="hero-q" class="sr-only">Qué trabajo buscas</label>
            <div class="flex flex-col gap-2 rounded-xl border border-gray-300 bg-white p-2 shadow-sm focus-within:border-emerald-600 sm:flex-row sm:items-center">
              <div class="flex flex-1 items-center gap-3 px-3">
                <PhMagnifyingGlass :size="20" class="shrink-0 text-gray-500" />
                <input
                  id="hero-q"
                  v-model="heroQuery"
                  type="text"
                  autocomplete="off"
                  placeholder="Ej. algo de diseño, remoto, para empezar"
                  class="w-full min-w-0 bg-transparent py-2.5 text-base text-gray-900 placeholder:text-gray-500 focus:outline-none"
                />
              </div>
              <button type="submit" class="press inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg bg-emerald-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-emerald-700">
                <PhSparkle :size="18" weight="fill" />
                Buscar con IA
              </button>
            </div>
          </form>

          <div v-if="!isLoggedIn()" class="rise mt-5" style="--i: 3">
            <router-link to="/register" class="press inline-flex items-center gap-1.5 text-sm font-medium text-gray-900 underline decoration-gray-300 underline-offset-4 hover:decoration-emerald-600">
              Crear cuenta
              <PhArrowRight :size="16" />
            </router-link>
          </div>
        </div>

        <div class="rise relative" style="--i: 2">
          <div class="aspect-[5/6] overflow-hidden rounded-xl bg-gray-100 shadow-lg lg:aspect-[4/5]">
            <img
              src="/img/ascenso.jpg"
              width="1400"
              height="1100"
              alt="Fachada de concreto blanco que se eleva contra un cielo oscuro"
              fetchpriority="high"
              class="settle h-full w-full object-cover"
            />
          </div>
        </div>
      </section>

      <!-- Qué hace la IA: bento de 4 celdas, con ritmo (ancha + angosta, angosta + ancha) -->
      <section class="mx-auto max-w-7xl px-4 py-24 sm:px-6 md:py-32">
        <h2 v-reveal class="max-w-2xl text-3xl font-semibold leading-tight tracking-tight text-gray-950 md:text-5xl">La IA hace el trabajo pesado de buscar</h2>

        <div class="mt-12 grid gap-4 md:grid-cols-6">
          <article v-reveal="0" class="flex flex-col justify-between gap-10 rounded-xl bg-emerald-50 p-7 md:col-span-4 md:min-h-[19rem] md:p-9">
            <div>
              <PhMagnifyingGlass :size="28" class="text-emerald-700" />
              <h3 class="mt-5 text-2xl font-semibold tracking-tight text-gray-950">Busca como hablas</h3>
              <p class="mt-3 max-w-[46ch] leading-relaxed text-gray-700">Escribe lo que quieres con tus palabras. La IA entiende el significado, no solo las palabras exactas.</p>
            </div>
            <div class="flex items-center gap-3 rounded-lg border border-emerald-200 bg-white px-4 py-3 text-gray-600">
              <PhSparkle :size="18" class="shrink-0 text-emerald-700" />
              <span class="truncate">algo de diseño, remoto, para empezar</span>
            </div>
          </article>

          <div v-reveal="1" class="min-h-64 overflow-hidden rounded-xl bg-gray-100 md:col-span-2">
            <img src="/img/oficio.jpg" width="1200" height="900" loading="lazy" alt="Persona sirviendo café con una tetera en un mostrador" class="h-full w-full object-cover" />
          </div>

          <article v-reveal="0" class="flex flex-col justify-between gap-10 rounded-xl bg-zinc-900 p-7 text-zinc-50 md:col-span-2 md:p-8">
            <PhTarget :size="28" />
            <div>
              <h3 class="text-2xl font-semibold tracking-tight">Mide tu encaje</h3>
              <p class="mt-3 leading-relaxed text-zinc-300">Cada oferta muestra qué tanto coincide con tu perfil y qué habilidades te faltan.</p>
            </div>
          </article>

          <article v-reveal="1" class="flex flex-col justify-between gap-10 rounded-xl border border-gray-200 bg-gradient-to-br from-emerald-100 via-white to-white p-7 md:col-span-4 md:p-9">
            <PhPencilLine :size="28" class="text-emerald-700" />
            <div>
              <h3 class="text-2xl font-semibold tracking-tight text-gray-950">Una carta lista para revisar</h3>
              <p class="mt-3 max-w-[50ch] leading-relaxed text-gray-700">La IA redacta un borrador con tu perfil y tú lo corriges. Las empresas también pulen sus vacantes con ella.</p>
            </div>
          </article>
        </div>
      </section>

      <!-- Categorías: tira horizontal con snap -->
      <section id="categorias" class="border-y border-gray-200 bg-gray-50 py-20 md:py-24">
        <div class="mx-auto max-w-7xl px-4 sm:px-6">
          <h2 v-reveal class="text-3xl font-semibold tracking-tight text-gray-950 md:text-4xl">Explora por área</h2>
        </div>
        <ul class="mx-auto mt-10 flex max-w-7xl snap-x snap-mandatory gap-3 overflow-x-auto px-4 pb-2 [scrollbar-width:none] sm:px-6">
          <li v-for="cat in categories" :key="cat.name" class="snap-start">
            <router-link
              :to="{ path: '/jobs', query: { q: cat.query } }"
              class="press group flex items-center gap-2.5 whitespace-nowrap rounded-full border border-gray-300 bg-white px-5 py-3 text-sm font-medium text-gray-800 hover:border-emerald-600 hover:text-emerald-700"
            >
              <component :is="cat.icon" :size="20" class="text-gray-500 group-hover:text-emerald-700" />
              {{ cat.name }}
            </router-link>
          </li>
        </ul>
      </section>

      <!-- Ofertas recientes: título a la izquierda, lista a la derecha -->
      <section id="ofertas" class="mx-auto grid max-w-7xl gap-12 px-4 py-24 sm:px-6 md:py-32 lg:grid-cols-[0.8fr_1.6fr] lg:gap-20">
        <div v-reveal class="lg:sticky lg:top-28 lg:self-start">
          <h2 class="text-3xl font-semibold tracking-tight text-gray-950 md:text-4xl">Ofertas recientes</h2>
          <p class="mt-4 max-w-[34ch] leading-relaxed text-gray-600">Las vacantes más nuevas, con el pago a la vista.</p>
          <router-link to="/jobs" class="press mt-7 inline-flex items-center gap-2 rounded-lg border border-gray-300 px-5 py-2.5 text-sm font-medium text-gray-900 hover:bg-gray-100">
            Ver todas
            <PhArrowRight :size="16" />
          </router-link>
        </div>

        <div>
          <!-- cargando: barras con la forma de la lista final -->
          <ul v-if="loading" class="space-y-3" aria-busy="true" aria-label="Cargando ofertas">
            <li v-for="n in 4" :key="n" class="shimmer h-24 rounded-xl"></li>
          </ul>

          <div v-else-if="error" class="rounded-xl border border-gray-200 bg-gray-50 p-8">
            <p class="font-medium text-gray-900">No pudimos cargar las ofertas.</p>
            <p class="mt-1 text-sm text-gray-600">Revisa tu conexión e inténtalo de nuevo.</p>
            <button type="button" class="press mt-5 rounded-lg bg-emerald-600 px-4 py-2 text-sm font-medium text-white hover:bg-emerald-700" @click="loadJobs">Reintentar</button>
          </div>

          <div v-else-if="featuredJobs.length === 0" class="rounded-xl border border-dashed border-gray-300 p-10">
            <PhBriefcase :size="28" class="text-gray-400" />
            <p class="mt-4 text-lg font-medium text-gray-900">Todavía no hay ofertas publicadas.</p>
            <p class="mt-1 max-w-[46ch] text-gray-600">Las primeras empresas están por llegar. Crea tu perfil y te avisamos cuando haya vacantes para ti.</p>
            <router-link :to="isEmployer ? '/jobs/create' : '/register'" class="press mt-6 inline-flex rounded-lg bg-emerald-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-emerald-700">
              {{ isEmployer ? 'Publicar vacante' : 'Crear cuenta' }}
            </router-link>
          </div>

          <ul v-else class="divide-y divide-gray-200 rounded-xl border border-gray-200 bg-white">
            <li v-for="(job, i) in featuredJobs" :key="job._id" v-reveal="i">
              <router-link :to="`/jobs/${job._id}`" class="group flex items-center gap-4 p-5 transition-colors hover:bg-gray-50 sm:gap-5 sm:p-6">
                <span class="grid h-12 w-12 shrink-0 place-items-center rounded-lg bg-emerald-100 text-lg font-semibold text-emerald-700">{{ initialOf(job) }}</span>
                <span class="min-w-0 flex-1">
                  <span class="block truncate text-base font-semibold text-gray-950 group-hover:text-emerald-700">{{ job.title }}</span>
                  <span class="mt-0.5 block truncate text-sm text-gray-600">{{ companyName(job) }}<template v-if="job.isRemote"> (remoto)</template></span>
                  <span v-if="job.tags?.length" class="mt-2 flex flex-wrap gap-1.5">
                    <span v-for="t in job.tags.slice(0, 3)" :key="t" class="rounded-full bg-gray-100 px-2.5 py-0.5 text-xs text-gray-700">{{ t }}</span>
                  </span>
                </span>
                <span class="hidden shrink-0 text-right font-mono text-sm tabular-nums text-gray-900 sm:block">{{ formatSalary(job) }}</span>
                <PhArrowRight :size="18" class="shrink-0 text-gray-400 transition-transform group-hover:translate-x-1 group-hover:text-emerald-700" />
              </router-link>
            </li>
          </ul>
        </div>
      </section>

      <!-- Cómo funciona: foto alta a la izquierda, pasos a la derecha -->
      <section id="como-funciona" class="border-t border-gray-200 bg-gray-50">
        <div class="mx-auto grid max-w-7xl items-center gap-12 px-4 py-24 sm:px-6 md:py-32 lg:grid-cols-[0.85fr_1.15fr] lg:gap-20">
          <div v-reveal class="aspect-[4/3] overflow-hidden rounded-xl bg-gray-200 lg:aspect-[4/5]">
            <img src="/img/amanecer.jpg" width="1200" height="900" loading="lazy" alt="Amanecer sobre el mar con un muelle y una gaviota en vuelo" class="h-full w-full object-cover" />
          </div>
          <div>
            <h2 v-reveal class="max-w-xl text-3xl font-semibold leading-tight tracking-tight text-gray-950 md:text-5xl">Del perfil a la entrevista</h2>
            <ol class="mt-12 space-y-10 border-l border-gray-300 pl-8">
              <li v-for="(step, i) in steps" :key="step.title" v-reveal="i" class="relative">
                <span class="absolute -left-[2.45rem] top-0.5 grid h-9 w-9 place-items-center rounded-lg border border-gray-300 bg-white text-gray-700">
                  <component :is="step.icon" :size="18" />
                </span>
                <h3 class="text-xl font-semibold tracking-tight text-gray-950">{{ step.title }}</h3>
                <p class="mt-2 max-w-[50ch] leading-relaxed text-gray-600">{{ step.text }}</p>
              </li>
            </ol>
          </div>
        </div>
      </section>

      <!-- Cierre -->
      <section class="mx-auto max-w-7xl px-4 py-24 sm:px-6 md:py-32">
        <div v-reveal class="grid items-end gap-10 rounded-xl bg-emerald-700 p-8 text-white md:p-14 lg:grid-cols-[1.3fr_0.7fr]">
          <div>
            <h2 class="max-w-xl text-3xl font-semibold leading-tight tracking-tight md:text-5xl">Publica una vacante y encuentra talento joven</h2>
            <p class="mt-5 max-w-[48ch] leading-relaxed text-emerald-50">Describe el puesto y la IA te ayuda a pulirlo. Recibe postulaciones de estudiantes con correo universitario.</p>
          </div>
          <div class="flex flex-col gap-3 sm:flex-row lg:flex-col">
            <router-link :to="isEmployer ? '/jobs/create' : '/empresas'" class="press inline-flex items-center justify-center whitespace-nowrap rounded-lg bg-white px-6 py-3 text-sm font-medium text-emerald-800 hover:bg-emerald-50">
              Publicar vacante
            </router-link>
            <router-link v-if="!isLoggedIn()" to="/register" class="press inline-flex items-center justify-center whitespace-nowrap rounded-lg border border-emerald-200/60 px-6 py-3 text-sm font-medium text-white hover:bg-emerald-600">
              Crear cuenta
            </router-link>
          </div>
        </div>
      </section>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import {
  PhMagnifyingGlass,
  PhSparkle,
  PhTarget,
  PhPencilLine,
  PhArrowRight,
  PhBriefcase,
  PhCode,
  PhPaintBrush,
  PhMegaphone,
  PhChartBar,
  PhPenNib,
  PhChalkboardTeacher,
  PhTranslate,
  PhGear,
  PhUserCircle,
  PhNotePencil
} from '@phosphor-icons/vue'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import { useAuth } from '../composables/useAuth'

const router = useRouter()
const { isLoggedIn, user } = useAuth()

const heroQuery = ref('')
const jobs = ref([])
const loading = ref(true)
const error = ref(false)

const isEmployer = computed(() => user.value?.role === 'employer')

const loadJobs = async () => {
  loading.value = true
  error.value = false
  try {
    jobs.value = (await JobService.getAll({ limit: 5 })) || []
  } catch (err) {
    error.value = true
    console.error(err)
  } finally {
    loading.value = false
  }
}
onMounted(loadJobs)

const featuredJobs = computed(() => jobs.value.slice(0, 5))
const companyName = job => job.company || 'Empresa'
const initialOf = job => companyName(job).charAt(0).toUpperCase()

const formatSalary = job => {
  const s = job.salaryRange
  if (!s || (s.min == null && s.max == null)) return 'A convenir'
  const range = s.min != null && s.max != null ? `$${s.min}-${s.max}` : `$${s.min ?? s.max}`
  return `${range} ${s.currency || ''}/${s.type || 'hora'}`.replace('  ', ' ')
}

// La búsqueda de la portada abre la lista con la IA activada (si no hay sesión, la lista cae a la búsqueda normal)
const goSearch = () => {
  const q = heroQuery.value.trim()
  router.push({ path: '/jobs', query: q ? { q, ai: '1' } : {} })
}

const categories = [
  { name: 'Desarrollo de software', query: 'desarrollo', icon: PhCode },
  { name: 'Diseño UX/UI', query: 'diseño', icon: PhPaintBrush },
  { name: 'Marketing digital', query: 'marketing', icon: PhMegaphone },
  { name: 'Datos e IA', query: 'datos', icon: PhChartBar },
  { name: 'Contenido', query: 'contenido', icon: PhPenNib },
  { name: 'Negocios', query: 'negocios', icon: PhBriefcase },
  { name: 'Tutorías', query: 'tutor', icon: PhChalkboardTeacher },
  { name: 'Idiomas', query: 'traducción', icon: PhTranslate },
  { name: 'Ingeniería', query: 'ingeniería', icon: PhGear }
]

const steps = [
  { title: 'Crea tu perfil', text: 'Regístrate con tu correo universitario y agrega tus habilidades.', icon: PhUserCircle },
  { title: 'Busca con IA', text: 'Describe lo que quieres y revisa qué tanto encajas con cada oferta.', icon: PhMagnifyingGlass },
  { title: 'Postúlate y da seguimiento', text: 'Envía tu carta y recibe un aviso cada vez que cambie tu estado.', icon: PhNotePencil }
]
</script>
