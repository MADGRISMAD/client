<template>
  <DefaultLayout variant="paper">
    <div class="paper font-grotesk text-ink bg-paper overflow-x-clip">
      <!-- ============ CABECERA EDITORIAL ============ -->
      <section class="px-4 sm:px-6">
        <div class="max-w-7xl mx-auto">
          <div class="flex flex-wrap items-center justify-between gap-x-6 gap-y-1 border-b border-ink py-3 font-mono text-[11px] uppercase tracking-wider">
            <span>Bolsa de trabajo universitaria</span>
            <span class="hidden sm:inline">Freelance · Remoto · Por proyecto</span>
            <span>{{ today }}</span>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-12 gap-10 lg:gap-6 pt-10 md:pt-16 pb-12 md:pb-20">
            <h1 class="lg:col-span-9 font-display leading-[0.88] tracking-[-0.02em] text-[clamp(3.6rem,11.5vw,11.5rem)]">
              Trabaja antes<br />
              de <em class="text-signal">graduarte.</em>
            </h1>

            <!-- Datos en vivo -->
            <dl class="lg:col-span-3 lg:pt-6 grid grid-cols-3 lg:grid-cols-1 gap-x-4 border-t border-ink lg:border-t-0 lg:border-l lg:pl-6">
              <div v-for="stat in stats" :key="stat.label" class="py-4 lg:py-5 lg:border-b border-ink/20 last:border-b-0">
                <dt class="font-mono text-[11px] uppercase tracking-wider text-ink/60">{{ stat.label }}</dt>
                <dd class="mt-1 font-display text-5xl md:text-6xl leading-none tabular-nums">
                  {{ loading ? '··' : stat.value }}
                </dd>
              </div>
            </dl>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 lg:gap-6 pb-16 md:pb-24">
            <p class="lg:col-span-4 text-lg md:text-xl leading-snug max-w-md">
              Proyectos pagados con empresas reales, a la medida de tu horario de clases.
              Postúlate, entrega y suma puntos para tu universidad.
            </p>

            <form class="lg:col-span-8 lg:col-start-5" @submit.prevent="goSearch()">
              <label for="hero-search" class="font-mono text-[11px] uppercase tracking-wider text-ink/60">¿Qué sabes hacer?</label>
              <div class="mt-2 flex items-stretch border-2 border-ink bg-paper focus-within:shadow-[6px_6px_0_0_var(--color-ink)] transition-shadow">
                <input
                  id="hero-search"
                  v-model="heroQuery"
                  type="text"
                  autocomplete="off"
                  placeholder="React, ilustración, Excel, inglés…"
                  class="min-w-0 flex-1 bg-transparent px-4 md:px-5 py-4 text-lg md:text-xl placeholder:text-ink/35 focus:outline-none"
                />
                <button type="submit" class="shrink-0 bg-ink px-5 md:px-8 text-paper font-semibold hover:bg-signal hover:text-ink transition-colors">
                  Buscar<span class="hidden sm:inline"> ofertas</span> →
                </button>
              </div>
              <div class="mt-4 flex flex-wrap gap-x-5 gap-y-2 font-mono text-sm">
                <button
                  v-for="term in popularSearches"
                  :key="term"
                  type="button"
                  class="underline decoration-ink/30 underline-offset-4 hover:decoration-signal hover:text-signal"
                  @click="goSearch(term)"
                >
                  {{ term }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </section>

      <!-- ============ TICKER ============ -->
      <div class="ticker bg-ink text-paper border-y border-ink" aria-hidden="true">
        <div class="ticker-track py-3 font-mono text-sm">
          <span v-for="(item, i) in [...tickerItems, ...tickerItems]" :key="i" class="flex items-center whitespace-nowrap">
            <span class="px-6">{{ item }}</span>
            <span class="text-signal">✶</span>
          </span>
        </div>
      </div>

      <!-- ============ 01 · TABLÓN ============ -->
      <section id="ofertas" class="px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto">
          <SectionHead number="01" title="El tablón" kicker="Ofertas abiertas ahora mismo" />

          <div class="mt-10 flex flex-wrap gap-x-6 gap-y-2 font-mono text-sm" role="tablist">
            <button
              v-for="f in filterOptions"
              :key="f.key"
              type="button"
              role="tab"
              :aria-selected="activeFilter === f.key"
              :class="[
                'pb-1 border-b-2 transition-colors',
                activeFilter === f.key ? 'border-signal text-ink' : 'border-transparent text-ink/50 hover:text-ink'
              ]"
              @click="activeFilter = f.key"
            >
              {{ f.label }} <sup class="text-[10px]">{{ filterCount(f.key) }}</sup>
            </button>
          </div>

          <!-- Encabezado de tabla -->
          <div class="mt-6 hidden md:grid grid-cols-[3rem_1fr_9rem_11rem_8rem_2rem] gap-4 border-y border-ink py-2 font-mono text-[11px] uppercase tracking-wider text-ink/60">
            <span>Nº</span><span>Puesto</span><span>Modalidad</span><span>Pago</span><span>Duración</span><span></span>
          </div>

          <div v-if="loading" class="mt-6 md:mt-0 border-y md:border-t-0 border-ink py-10 font-mono text-sm">
            Cargando ofertas<span class="blink">_</span>
          </div>

          <div v-else-if="error" class="mt-6 md:mt-0 border-y md:border-t-0 border-ink py-12">
            <p class="font-display text-4xl">No pudimos cargar el tablón.</p>
            <p class="mt-2 text-ink/60">Puede ser tu conexión o el servidor. Inténtalo otra vez.</p>
            <button type="button" class="mt-6 border-2 border-ink px-5 py-2.5 font-semibold hover:bg-ink hover:text-paper transition-colors" @click="loadJobs">
              Reintentar
            </button>
          </div>

          <div v-else-if="!boardJobs.length" class="mt-6 md:mt-0 border-y md:border-t-0 border-ink py-12">
            <p class="font-display text-4xl">Nada por aquí con este filtro.</p>
            <p class="mt-2 text-ink/60">Prueba con otro o revisa todas las ofertas.</p>
          </div>

          <ol v-else class="mt-6 md:mt-0 border-t md:border-t-0 border-ink">
            <li v-for="(job, i) in boardJobs" :key="job._id">
              <router-link
                :to="`/jobs/${job._id}`"
                class="group grid grid-cols-[2.25rem_1fr_1.5rem] md:grid-cols-[3rem_1fr_9rem_11rem_8rem_2rem] gap-x-4 gap-y-1 items-baseline border-b border-ink py-5 md:py-6 transition-colors hover:bg-signal"
              >
                <span class="font-mono text-sm text-ink/50 group-hover:text-ink">{{ String(i + 1).padStart(2, '0') }}</span>
                <span class="min-w-0">
                  <span class="block font-display text-3xl md:text-4xl leading-tight">
                    {{ job.title }}
                    <span v-if="job.highlighted" class="align-middle ml-1 inline-block -translate-y-1 bg-ink px-1.5 py-0.5 font-mono text-[10px] uppercase tracking-wider text-paper">Top</span>
                  </span>
                  <span class="mt-1 block text-ink/60 group-hover:text-ink">
                    {{ companyName(job) }}<template v-if="job.tags?.length"> — {{ job.tags.slice(0, 3).join(', ') }}</template>
                  </span>
                  <!-- Datos en móvil -->
                  <span class="md:hidden mt-3 flex flex-wrap gap-x-4 gap-y-1 font-mono text-xs">
                    <span>{{ job.isRemote ? 'Remoto' : 'Presencial' }}</span>
                    <span>{{ formatSalary(job) }}</span>
                    <span>{{ job.duration || 'Flexible' }}</span>
                  </span>
                </span>
                <span class="hidden md:block font-mono text-sm">{{ job.isRemote ? 'Remoto' : 'Presencial' }}</span>
                <span class="hidden md:block font-mono text-sm">{{ formatSalary(job) }}</span>
                <span class="hidden md:block font-mono text-sm">{{ job.duration || 'Flexible' }}</span>
                <span class="text-xl transition-transform group-hover:translate-x-1">→</span>
              </router-link>
            </li>
          </ol>

          <div class="mt-8 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <p v-if="!loading && !error && filteredByTab.length > boardJobs.length" class="font-mono text-sm text-ink/60">
              Mostrando {{ boardJobs.length }} de {{ filteredByTab.length }}
            </p>
            <span v-else></span>
            <router-link to="/jobs" class="inline-flex items-center justify-center gap-2 bg-ink px-6 py-3.5 font-semibold text-paper hover:bg-signal hover:text-ink transition-colors">
              Ver el tablón completo →
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ 02 · ÁREAS ============ -->
      <section id="categorias" class="px-4 sm:px-6 pb-20 md:pb-28">
        <div class="max-w-7xl mx-auto">
          <SectionHead number="02" title="Por área" kicker="Elige por lo que estudias, o por lo que te gusta" />

          <p class="mt-10 font-display text-[clamp(2.6rem,7vw,6.5rem)] leading-[1.02] tracking-[-0.01em]">
            <template v-for="(area, i) in areasWithCount" :key="area.name">
              <router-link
                :to="{ path: '/jobs', query: { q: area.query } }"
                class="whitespace-nowrap hover:text-signal hover:italic transition-colors"
              >{{ area.name }}<sup v-if="area.count" class="font-mono text-[0.22em] align-super ml-1 not-italic text-signal">{{ area.count }}</sup></router-link>
              <span v-if="i < areasWithCount.length - 1" class="text-ink/25 mx-[0.15em]"> / </span>
            </template>
          </p>
        </div>
      </section>

      <!-- ============ 03 · CÓMO FUNCIONA ============ -->
      <section id="como-funciona" class="px-4 sm:px-6 py-20 md:py-28 border-t border-ink">
        <div class="max-w-7xl mx-auto">
          <SectionHead number="03" title="Cómo funciona" kicker="Tres pasos, sin letra pequeña" />

          <ol class="mt-12 grid md:grid-cols-3 border-t border-ink">
            <li
              v-for="(step, i) in steps"
              :key="step.title"
              :class="['py-8 md:py-10 md:px-8 border-b md:border-b-0 border-ink', i > 0 ? 'md:border-l' : 'md:pl-0']"
            >
              <span class="font-display italic text-6xl text-signal">{{ step.numeral }}</span>
              <h3 class="mt-6 text-2xl font-bold tracking-tight">{{ step.title }}</h3>
              <p class="mt-3 text-ink/70 max-w-sm leading-relaxed">{{ step.text }}</p>
              <p class="mt-6 font-mono text-xs uppercase tracking-wider text-ink/50">+{{ step.points }} pts</p>
            </li>
          </ol>
        </div>
      </section>

      <!-- ============ 04 · CLASIFICACIÓN ============ -->
      <section class="bg-ink text-paper px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-12">
          <div class="lg:col-span-5">
            <SectionHead number="04" title="Clasificación" kicker="Temporada en curso" dark />
            <p class="mt-8 text-lg text-paper/70 max-w-md leading-relaxed">
              Cada proyecto entregado suma puntos para ti, tu universidad y tu carrera.
              Las empresas ven primero a quien va arriba.
            </p>
          </div>

          <div class="lg:col-span-7">
            <div class="flex gap-6 font-mono text-sm border-b border-paper/20" role="tablist">
              <button
                v-for="tab in rankingTabs"
                :key="tab.key"
                type="button"
                role="tab"
                :aria-selected="activeRanking === tab.key"
                :class="[
                  '-mb-px pb-3 border-b-2 transition-colors',
                  activeRanking === tab.key ? 'border-signal text-paper' : 'border-transparent text-paper/50 hover:text-paper'
                ]"
                @click="activeRanking = tab.key"
              >
                {{ tab.label }}
              </button>
            </div>

            <ol>
              <li
                v-for="(entry, i) in currentRanking"
                :key="entry.name"
                class="grid grid-cols-[3rem_1fr_auto] md:grid-cols-[4rem_1fr_auto] items-center gap-4 border-b border-paper/20 py-6"
              >
                <span :class="['font-display text-5xl md:text-6xl leading-none', i === 0 ? 'text-signal' : 'text-paper/40']">{{ i + 1 }}</span>
                <span class="min-w-0">
                  <span class="block text-xl md:text-2xl font-semibold truncate">{{ entry.name }}</span>
                  <span class="block font-mono text-xs uppercase tracking-wider text-paper/50 mt-1">{{ entry.detail }}</span>
                </span>
                <span class="font-mono text-lg md:text-xl tabular-nums">{{ entry.points.toLocaleString('es-MX') }}<span class="text-paper/50 text-sm"> pts</span></span>
              </li>
            </ol>
          </div>
        </div>
      </section>

      <!-- ============ EMPRESAS ============ -->
      <section class="bg-signal px-4 sm:px-6 py-20 md:py-24">
        <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-10 items-end">
          <h2 class="lg:col-span-8 font-display text-[clamp(3rem,8vw,8rem)] leading-[0.9] tracking-[-0.02em]">
            ¿Buscas talento<br /><em>con hambre?</em>
          </h2>
          <div class="lg:col-span-4">
            <p class="text-lg leading-snug">
              Publica una vacante y recibe postulaciones de estudiantes que quieren demostrar de qué son capaces.
            </p>
            <router-link
              :to="isEmployer ? '/jobs/create' : '/empresas'"
              class="mt-6 inline-flex items-center gap-2 bg-ink px-6 py-3.5 font-semibold text-paper hover:bg-paper hover:text-ink transition-colors"
            >
              {{ isEmployer ? 'Publicar vacante' : 'Ver planes para empresas' }} →
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ 05 · PREGUNTAS ============ -->
      <section class="px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-10">
          <div class="lg:col-span-4">
            <SectionHead number="05" title="Preguntas" kicker="Lo que nos preguntan siempre" />
          </div>
          <div class="lg:col-span-8 border-t border-ink">
            <details v-for="item in faq" :key="item.q" class="faq border-b border-ink">
              <summary class="flex cursor-pointer list-none items-center justify-between gap-6 py-6 text-xl md:text-2xl font-semibold tracking-tight hover:text-signal">
                {{ item.q }}
                <span class="faq-icon shrink-0 font-mono text-2xl transition-transform">+</span>
              </summary>
              <p class="pb-6 max-w-2xl text-ink/70 leading-relaxed">{{ item.a }}</p>
            </details>
          </div>
        </div>
      </section>

      <!-- ============ CIERRE ============ -->
      <section class="px-4 sm:px-6 pb-24 md:pb-32">
        <div class="max-w-7xl mx-auto border-t border-ink pt-16 md:pt-20 flex flex-col lg:flex-row lg:items-end justify-between gap-10">
          <p class="font-display text-[clamp(2.8rem,7vw,7rem)] leading-[0.92] tracking-[-0.02em] max-w-5xl">
            Tu portafolio no se va a <em class="text-signal">llenar solo.</em>
          </p>
          <router-link
            :to="studentCta.to"
            class="shrink-0 self-start lg:self-auto inline-flex items-center gap-2 border-2 border-ink bg-ink px-7 py-4 text-lg font-semibold text-paper hover:bg-paper hover:text-ink transition-colors"
          >
            {{ studentCta.label }} →
          </router-link>
        </div>
      </section>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, computed, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import { useAuth } from '../composables/useAuth'

// Encabezado numerado de cada sección
const SectionHead = props =>
  h('div', { class: ['flex items-baseline gap-4 md:gap-6 border-b pb-4', props.dark ? 'border-paper/20' : 'border-ink'] }, [
    h('span', { class: 'font-mono text-sm text-signal' }, props.number),
    h('div', { class: 'min-w-0' }, [
      h('h2', { class: 'font-display text-5xl md:text-7xl leading-none tracking-[-0.01em]' }, props.title),
      h('p', { class: ['mt-2 font-mono text-xs uppercase tracking-wider', props.dark ? 'text-paper/50' : 'text-ink/50'] }, props.kicker)
    ])
  ])
SectionHead.props = ['number', 'title', 'kicker', 'dark']

const router = useRouter()
const { user, isLoggedIn } = useAuth()

const jobs = ref([])
const loading = ref(true)
const error = ref('')
const heroQuery = ref('')
const activeFilter = ref('all')
const activeRanking = ref('students')

const isEmployer = computed(() => user.value?.role === 'employer')
const studentCta = computed(() =>
  isLoggedIn() ? { to: '/jobs', label: 'Explorar ofertas' } : { to: '/register', label: 'Crear mi perfil' }
)

const today = new Date().toLocaleDateString('es-MX', { day: 'numeric', month: 'long', year: 'numeric' })

// ---------- Datos ----------
const loadJobs = async () => {
  loading.value = true
  error.value = ''
  try {
    jobs.value = (await JobService.getAll()) || []
  } catch (err) {
    error.value = 'Error al cargar vacantes'
    console.error(err)
  } finally {
    loading.value = false
  }
}

onMounted(loadJobs)

const companyName = job => job.company || job.createdBy?.fullName || 'Empresa'

const formatSalary = job => {
  const s = job.salaryRange
  if (!s || (s.min == null && s.max == null)) return 'A convenir'
  const range = s.min != null && s.max != null ? `$${s.min}–${s.max}` : `$${s.min ?? s.max}`
  return s.type ? `${range}/${s.type}` : range
}

const pad = n => String(n).padStart(2, '0')

const stats = computed(() => [
  { label: 'Ofertas', value: pad(jobs.value.length) },
  { label: 'Empresas', value: pad(new Set(jobs.value.map(companyName)).size) },
  { label: 'Remotas', value: pad(jobs.value.filter(j => j.isRemote).length) }
])

// ---------- Tablón ----------
const filters = {
  all: () => true,
  remote: j => j.isRemote,
  highlighted: j => j.highlighted,
  project: j => j.salaryRange?.type === 'proyecto'
}

const filterOptions = [
  { key: 'all', label: 'Todas' },
  { key: 'remote', label: 'Remotas' },
  { key: 'highlighted', label: 'Destacadas' },
  { key: 'project', label: 'Por proyecto' }
]

const filterCount = key => jobs.value.filter(filters[key]).length

const filteredByTab = computed(() =>
  jobs.value
    .filter(filters[activeFilter.value])
    .sort((a, b) => Number(!!b.highlighted) - Number(!!a.highlighted))
)

const boardJobs = computed(() => filteredByTab.value.slice(0, 8))

const tickerItems = computed(() => {
  if (jobs.value.length) {
    return jobs.value.slice(0, 12).map(j => `${j.title} — ${companyName(j)} — ${formatSalary(j)}`)
  }
  return ['Freelance', 'Remoto', 'Por proyecto', 'Pagado', 'Para universitarios', 'Desde tu casa', 'Entre clases']
})

// ---------- Búsqueda ----------
const popularSearches = ['Vue', 'Figma', 'Python', 'Marketing', 'Redacción']

const goSearch = term => {
  const q = (term ?? heroQuery.value).trim()
  router.push({ path: '/jobs', query: q ? { q } : {} })
}

// ---------- Áreas ----------
const areas = [
  { name: 'Desarrollo', query: 'desarrollo', keywords: ['desarrollo', 'developer', 'frontend', 'backend', 'vue', 'react', 'javascript', 'python', 'software'] },
  { name: 'Diseño', query: 'diseño', keywords: ['diseño', 'ux', 'ui', 'figma', 'design'] },
  { name: 'Marketing', query: 'marketing', keywords: ['marketing', 'redes', 'social', 'seo', 'growth'] },
  { name: 'Datos', query: 'datos', keywords: ['datos', 'data', 'sql', 'análisis', 'analista'] },
  { name: 'Redacción', query: 'redacción', keywords: ['redacción', 'redactor', 'contenido', 'copy'] },
  { name: 'Video', query: 'video', keywords: ['video', 'edición', 'motion'] },
  { name: 'Negocios', query: 'negocios', keywords: ['negocios', 'ventas', 'finanzas', 'business'] },
  { name: 'Tutorías', query: 'tutor', keywords: ['tutor', 'clases', 'asesoría', 'enseñanza'] },
  { name: 'Idiomas', query: 'traducción', keywords: ['traducción', 'idiomas', 'inglés', 'english'] },
  { name: 'Ingeniería', query: 'ingeniería', keywords: ['ingeniería', 'cad', 'mecánica', 'industrial'] }
]

const areasWithCount = computed(() =>
  areas.map(area => ({
    ...area,
    count: jobs.value.filter(job => {
      const text = [job.title, job.description, ...(job.tags || [])].join(' ').toLowerCase()
      return area.keywords.some(k => text.includes(k))
    }).length
  }))
)

// ---------- Contenido ----------
const steps = [
  { numeral: 'i.', title: 'Crea tu perfil', points: 100, text: 'Regístrate con tu correo universitario y cuenta qué sabes hacer. Cinco minutos, no más.' },
  { numeral: 'ii.', title: 'Postúlate', points: 250, text: 'Filtra por área, modalidad o pago. Postúlate a lo que encaje con tu horario.' },
  { numeral: 'iii.', title: 'Entrega y sube', points: 500, text: 'Trabaja con la empresa, recibe feedback y suma puntos a tu perfil y a tu universidad.' }
]

const rankingTabs = [
  { key: 'students', label: 'Estudiantes' },
  { key: 'universities', label: 'Universidades' },
  { key: 'specialties', label: 'Carreras' }
]

const rankings = {
  students: [
    { name: 'Camila Torres', detail: 'Ingeniería de Software', points: 850 },
    { name: 'Iván Martínez', detail: 'Diseño UX/UI', points: 770 },
    { name: 'Lucía Gómez', detail: 'Marketing Digital', points: 740 }
  ],
  universities: [
    { name: 'UNAM', detail: 'Ciudad de México', points: 4320 },
    { name: 'IPN', detail: 'México', points: 3980 },
    { name: 'UDG', detail: 'Guadalajara', points: 3600 }
  ],
  specialties: [
    { name: 'Ingeniería de Software', detail: 'Tecnología', points: 7820 },
    { name: 'Diseño Gráfico', detail: 'Creatividad', points: 6150 },
    { name: 'Marketing Digital', detail: 'Comercial', points: 5940 }
  ]
}

const currentRanking = computed(() => rankings[activeRanking.value])

const faq = [
  { q: '¿Quién puede postularse?', a: 'Cualquier estudiante universitario con un correo institucional. No necesitas experiencia previa: muchas ofertas están pensadas justo para tu primer proyecto.' },
  { q: '¿Qué tipo de trabajos hay?', a: 'Trabajos freelance, remotos y por proyecto. Cada oferta indica modalidad, pago y duración antes de que te postules.' },
  { q: '¿Cómo funcionan los puntos?', a: 'Cada paso suma: completar tu perfil, postularte y, sobre todo, entregar proyectos. Tus puntos también cuentan para la clasificación de tu universidad y tu carrera.' },
  { q: 'Soy empresa, ¿cómo publico una vacante?', a: 'Crea una cuenta de empresa y elige un plan. El plan Básico es gratis e incluye dos ofertas activas.' }
]
</script>

<style scoped>
/* Grano de papel */
.paper {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='160' height='160'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='.85' numOctaves='3' stitchTiles='stitch'/%3E%3CfeColorMatrix values='0 0 0 0 0.08 0 0 0 0 0.08 0 0 0 0 0.07 0 0 0 .07 0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
}

.ticker {
  overflow: hidden;
}

.ticker-track {
  display: flex;
  width: max-content;
  animation: ticker 60s linear infinite;
}

.ticker:hover .ticker-track {
  animation-play-state: paused;
}

@keyframes ticker {
  to { transform: translateX(-50%); }
}

.blink {
  animation: blink 1s steps(1) infinite;
}

@keyframes blink {
  50% { opacity: 0; }
}

.faq[open] .faq-icon {
  transform: rotate(45deg);
}

.faq summary::-webkit-details-marker {
  display: none;
}

@media (prefers-reduced-motion: reduce) {
  .ticker-track,
  .blink {
    animation: none;
  }
}
</style>
