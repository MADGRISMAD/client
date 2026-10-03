<template>
  <DefaultLayout>
    <div class="font-nunito text-slate-800 bg-white overflow-x-clip">
      <!-- ============ HERO ============ -->
      <section class="relative px-4 sm:px-6 pt-12 pb-20 md:pt-20 md:pb-28">
        <!-- Manchas de color -->
        <div class="pointer-events-none absolute inset-0 -z-0">
          <div class="absolute -top-24 -left-24 h-80 w-80 rounded-full bg-emerald-100"></div>
          <div class="absolute top-40 -right-20 h-72 w-72 rounded-full bg-yellow-100"></div>
        </div>

        <div class="relative max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-14 lg:gap-10 items-center">
          <!-- Texto -->
          <div>
            <span class="inline-flex -rotate-2 items-center gap-2 rounded-xl border-2 border-slate-800 bg-yellow-300 px-3 py-1 text-sm font-extrabold shadow-[3px_3px_0_#1e293b]">
              🎓 Solo para universitarios
            </span>

            <h1 class="mt-6 text-[2.75rem] sm:text-6xl xl:text-7xl font-black leading-[1.02] tracking-tight">
              ¡Sube de nivel
              <span class="relative inline-block text-emerald-500">
                tu carrera
                <svg class="absolute -bottom-3 left-0 w-full" height="14" viewBox="0 0 200 14" preserveAspectRatio="none" aria-hidden="true">
                  <path d="M3 9c40-8 80-8 120-3s50 3 74-2" fill="none" stroke="#fbbf24" stroke-width="6" stroke-linecap="round" />
                </svg>
              </span>
              mientras estudias!
            </h1>

            <p class="mt-7 max-w-xl text-lg sm:text-xl font-semibold text-slate-500 leading-relaxed">
              Misiones reales con empresas reales: trabajos freelance, remotos y por proyecto.
              Gana dinero, junta XP y llena tu portafolio.
            </p>

            <!-- Buscador -->
            <form
              class="mt-8 flex flex-col sm:flex-row gap-3 rounded-3xl border-2 border-slate-200 bg-white p-3 shadow-[0_6px_0_#e2e8f0] focus-within:border-emerald-400 focus-within:shadow-[0_6px_0_#a7f3d0] transition"
              @submit.prevent="goSearch()"
            >
              <label class="relative flex-1">
                <span class="sr-only">Buscar ofertas</span>
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-2xl" aria-hidden="true">🔎</span>
                <input
                  v-model="heroQuery"
                  type="text"
                  autocomplete="off"
                  placeholder="¿Qué quieres hacer? Diseño, código, videos…"
                  class="w-full rounded-2xl bg-slate-50 py-4 pl-14 pr-4 text-lg font-bold placeholder:font-semibold placeholder:text-slate-400 focus:outline-none"
                />
              </label>
              <button type="submit" class="btn-3d btn-green px-8 py-4 text-lg">
                Buscar
              </button>
            </form>

            <div class="mt-5 flex flex-wrap gap-2">
              <button
                v-for="(term, i) in popularSearches"
                :key="term.label"
                type="button"
                :class="['chip', chipColors[i % chipColors.length]]"
                @click="goSearch(term.label)"
              >
                {{ term.emoji }} {{ term.label }}
              </button>
            </div>
          </div>

          <!-- Mascota + stickers -->
          <div class="relative mx-auto w-full max-w-md aspect-square">
            <div class="absolute inset-6 rounded-[3rem] bg-emerald-400 rotate-3"></div>
            <div class="absolute inset-6 rounded-[3rem] bg-emerald-300 -rotate-3"></div>

            <svg class="mascot absolute inset-0 m-auto w-3/4" viewBox="0 0 200 200" aria-label="Mascota de internships.gg">
              <ellipse cx="100" cy="190" rx="58" ry="7" fill="#065f46" opacity=".25" />
              <!-- brazo izquierdo -->
              <ellipse cx="40" cy="132" rx="13" ry="20" fill="#059669" transform="rotate(25 40 132)" />
              <!-- cuerpo -->
              <rect x="38" y="66" width="124" height="120" rx="56" fill="#047857" />
              <rect x="38" y="58" width="124" height="120" rx="56" fill="#10b981" />
              <ellipse cx="100" cy="140" rx="40" ry="30" fill="#6ee7b7" />
              <!-- brazo saludando -->
              <g class="wave">
                <ellipse cx="170" cy="112" rx="12" ry="20" fill="#047857" transform="rotate(-50 170 112)" />
                <ellipse cx="168" cy="108" rx="12" ry="20" fill="#10b981" transform="rotate(-50 168 108)" />
              </g>
              <!-- ojos -->
              <circle cx="78" cy="102" r="17" fill="#fff" />
              <circle cx="122" cy="102" r="17" fill="#fff" />
              <circle cx="82" cy="105" r="9" fill="#0f172a" />
              <circle cx="126" cy="105" r="9" fill="#0f172a" />
              <circle cx="85" cy="101" r="3.2" fill="#fff" />
              <circle cx="129" cy="101" r="3.2" fill="#fff" />
              <!-- cachetes + boca -->
              <ellipse cx="60" cy="124" rx="8" ry="5" fill="#f9a8d4" />
              <ellipse cx="140" cy="124" rx="8" ry="5" fill="#f9a8d4" />
              <path d="M88 124q12 14 24 0" fill="#064e3b" />
              <path d="M94 129q6 5 12 0" fill="#fb7185" />
              <!-- birrete -->
              <rect x="72" y="46" width="56" height="18" rx="5" fill="#0f172a" />
              <polygon points="100,22 164,42 100,62 36,42" fill="#1e293b" />
              <path d="M100 42 L150 50 L150 76" fill="none" stroke="#fbbf24" stroke-width="3.5" stroke-linecap="round" stroke-linejoin="round" />
              <circle cx="150" cy="80" r="6" fill="#fbbf24" />
            </svg>

            <!-- Stickers -->
            <div class="sticker float-a absolute -top-2 left-0 sm:-left-6 rotate-[-8deg] bg-yellow-300">
              ⚡ +50 XP
            </div>
            <div class="sticker float-b absolute top-10 -right-1 sm:-right-8 rotate-[6deg] bg-orange-400 text-white">
              🔥 Racha de 7 días
            </div>
            <div class="sticker float-c absolute bottom-28 sm:bottom-16 -left-1 sm:-left-10 rotate-[4deg] bg-violet-500 text-white">
              🎉 ¡Te contrataron!
            </div>

            <!-- Mini oferta -->
            <router-link
              :to="heroJob.link"
              class="float-b absolute -bottom-6 right-0 sm:-right-6 w-64 rotate-[-3deg] rounded-2xl border-2 border-slate-800 bg-white p-4 shadow-[4px_4px_0_#1e293b] hover:rotate-0 transition-transform"
            >
              <div class="flex items-center gap-3">
                <span class="flex h-10 w-10 items-center justify-center rounded-xl bg-sky-400 text-lg font-black text-white">{{ heroJob.initial }}</span>
                <div class="min-w-0">
                  <p class="truncate font-extrabold leading-tight">{{ heroJob.title }}</p>
                  <p class="truncate text-sm font-bold text-slate-400">{{ heroJob.company }}</p>
                </div>
              </div>
              <div class="mt-3 flex items-center justify-between">
                <span class="rounded-lg bg-emerald-100 px-2 py-0.5 text-sm font-extrabold text-emerald-700">{{ heroJob.salary }}</span>
                <span class="text-sm font-extrabold text-emerald-600">Ver →</span>
              </div>
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ STATS ============ -->
      <section class="px-4 sm:px-6">
        <div class="max-w-7xl mx-auto grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div
            v-for="stat in stats"
            :key="stat.label"
            :class="['flex items-center gap-4 rounded-3xl border-2 p-5', stat.box]"
          >
            <span :class="['flex h-14 w-14 shrink-0 items-center justify-center rounded-2xl text-3xl', stat.iconBox]">{{ stat.emoji }}</span>
            <div>
              <p class="text-3xl font-black leading-none">{{ loading ? '…' : stat.value }}</p>
              <p class="mt-1 font-bold text-slate-500">{{ stat.label }}</p>
            </div>
          </div>
        </div>
      </section>

      <!-- ============ CATEGORÍAS ============ -->
      <section id="categorias" class="px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto">
          <div class="text-center max-w-2xl mx-auto">
            <h2 class="text-4xl md:text-5xl font-black tracking-tight">Elige tu mundo 🌍</h2>
            <p class="mt-3 text-lg font-semibold text-slate-500">Cada área tiene sus propias misiones. ¿Cuál es la tuya?</p>
          </div>

          <div class="mt-12 grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
            <router-link
              v-for="cat in categoriesWithCount"
              :key="cat.name"
              :to="{ path: '/jobs', query: { q: cat.query } }"
              :class="['tile group', cat.tile]"
            >
              <span class="block text-5xl transition-transform group-hover:scale-110 group-hover:-rotate-6">{{ cat.emoji }}</span>
              <span class="mt-4 block text-lg font-black leading-tight">{{ cat.name }}</span>
              <span class="mt-1 block text-sm font-bold opacity-70">
                {{ cat.count ? `${cat.count} ${cat.count === 1 ? 'misión' : 'misiones'}` : 'Ver misiones' }}
              </span>
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ OFERTAS ============ -->
      <section id="ofertas" class="bg-emerald-50 px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto">
          <div class="flex flex-col lg:flex-row lg:items-end justify-between gap-6">
            <div>
              <h2 class="text-4xl md:text-5xl font-black tracking-tight">Misiones disponibles ⚔️</h2>
              <p class="mt-3 text-lg font-semibold text-slate-500">Recién publicadas. Postúlate antes de que vuelen.</p>
            </div>
            <div class="flex flex-wrap gap-2 rounded-2xl bg-white p-1.5 border-2 border-emerald-100 self-start lg:self-auto">
              <button
                v-for="f in filterOptions"
                :key="f.key"
                type="button"
                :class="[
                  'rounded-xl px-4 py-2 text-sm font-extrabold transition',
                  activeFilter === f.key ? 'bg-emerald-500 text-white shadow-[0_3px_0_#047857]' : 'text-slate-500 hover:bg-slate-100'
                ]"
                @click="activeFilter = f.key"
              >
                {{ f.label }}
              </button>
            </div>
          </div>

          <!-- Cargando -->
          <div v-if="loading" class="mt-10 grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <div v-for="n in 6" :key="n" class="card animate-pulse">
              <div class="flex items-center gap-3">
                <div class="h-12 w-12 rounded-2xl bg-slate-200"></div>
                <div class="flex-1 space-y-2">
                  <div class="h-4 w-2/3 rounded-full bg-slate-200"></div>
                  <div class="h-3 w-1/3 rounded-full bg-slate-100"></div>
                </div>
              </div>
              <div class="mt-5 h-3 rounded-full bg-slate-100"></div>
              <div class="mt-2 h-3 w-4/5 rounded-full bg-slate-100"></div>
              <div class="mt-6 h-12 rounded-2xl bg-slate-100"></div>
            </div>
          </div>

          <!-- Error -->
          <div v-else-if="error" class="mt-10 card text-center py-12">
            <p class="text-5xl">😵</p>
            <p class="mt-4 text-2xl font-black">¡Ups! No pudimos cargar las misiones</p>
            <p class="mt-2 font-semibold text-slate-500">Revisa tu conexión e inténtalo otra vez.</p>
            <button type="button" class="btn-3d btn-green mt-6 px-6 py-3" @click="loadJobs">Reintentar</button>
          </div>

          <!-- Vacío -->
          <div v-else-if="!featuredJobs.length" class="mt-10 card text-center py-12">
            <p class="text-5xl">🦗</p>
            <p class="mt-4 text-2xl font-black">Nada por aquí… todavía</p>
            <p class="mt-2 font-semibold text-slate-500">Prueba otro filtro o mira todas las misiones.</p>
            <router-link to="/jobs" class="btn-3d btn-green mt-6 inline-block px-6 py-3">Ver todas</router-link>
          </div>

          <!-- Grid -->
          <div v-else class="mt-10 grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <article v-for="(job, i) in featuredJobs" :key="job._id" class="card flex flex-col">
              <div class="flex items-start justify-between gap-3">
                <div class="flex items-center gap-3 min-w-0">
                  <span :class="['flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl text-xl font-black text-white', avatarColors[i % avatarColors.length]]">
                    {{ companyName(job)[0]?.toUpperCase() || '?' }}
                  </span>
                  <div class="min-w-0">
                    <h3 class="truncate text-lg font-black leading-tight">{{ job.title }}</h3>
                    <p class="truncate font-bold text-slate-400">{{ companyName(job) }}</p>
                  </div>
                </div>
                <span v-if="job.highlighted" class="shrink-0 rounded-xl bg-yellow-300 px-2 py-1 text-xs font-black">⭐ TOP</span>
              </div>

              <p class="mt-4 font-semibold text-slate-500 line-clamp-2">{{ job.description }}</p>

              <div class="mt-4 flex flex-wrap gap-2">
                <span v-if="job.isRemote" class="rounded-xl bg-sky-100 px-2.5 py-1 text-sm font-extrabold text-sky-700">🏠 Remoto</span>
                <span v-if="job.duration" class="rounded-xl bg-violet-100 px-2.5 py-1 text-sm font-extrabold text-violet-700">⏳ {{ job.duration }}</span>
                <span
                  v-for="tag in (job.tags || []).slice(0, 2)"
                  :key="tag"
                  class="rounded-xl bg-slate-100 px-2.5 py-1 text-sm font-extrabold text-slate-600"
                >#{{ tag }}</span>
              </div>

              <div class="mt-auto pt-6 flex items-center justify-between gap-3">
                <span class="text-lg font-black text-emerald-600">💰 {{ formatSalary(job) }}</span>
                <router-link :to="`/jobs/${job._id}`" class="btn-3d btn-green px-5 py-2.5 text-sm">
                  Ver misión
                </router-link>
              </div>
            </article>
          </div>

          <div v-if="!loading && !error && jobs.length" class="mt-12 text-center">
            <router-link to="/jobs" class="btn-3d btn-white inline-block px-8 py-4 text-lg">
              Ver todas las misiones →
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ CÓMO FUNCIONA (camino) ============ -->
      <section id="como-funciona" class="px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-6xl mx-auto">
          <div class="text-center max-w-2xl mx-auto">
            <h2 class="text-4xl md:text-5xl font-black tracking-tight">Tu camino 🗺️</h2>
            <p class="mt-3 text-lg font-semibold text-slate-500">Cuatro niveles para pasar de estudiante a pro.</p>
          </div>

          <div class="relative mt-16">
            <!-- línea del camino (escritorio) -->
            <svg class="hidden md:block absolute left-0 top-12 w-full h-24" viewBox="0 0 1000 100" preserveAspectRatio="none" aria-hidden="true">
              <path d="M125 20 C 250 20, 250 80, 375 80 S 500 20, 625 20 S 750 80, 875 80" fill="none" stroke="#d1fae5" stroke-width="10" stroke-linecap="round" stroke-dasharray="2 22" />
            </svg>

            <ol class="relative grid grid-cols-1 md:grid-cols-4 gap-10 md:gap-6">
              <li
                v-for="(step, i) in steps"
                :key="step.title"
                :class="['flex flex-col items-center text-center', i % 2 ? 'md:mt-16' : '', i % 2 ? 'translate-x-10 md:translate-x-0' : '-translate-x-10 md:translate-x-0']"
              >
                <span :class="['level-node', step.node]">
                  <span class="text-4xl">{{ step.emoji }}</span>
                  <span class="absolute -top-2 -right-2 flex h-8 w-8 items-center justify-center rounded-full border-2 border-white bg-slate-800 text-sm font-black text-white">{{ i + 1 }}</span>
                </span>
                <p class="mt-5 text-xs font-black uppercase tracking-widest text-emerald-500">Nivel {{ i + 1 }} · +{{ step.xp }} XP</p>
                <h3 class="mt-1 text-xl font-black">{{ step.title }}</h3>
                <p class="mt-2 max-w-[16rem] font-semibold text-slate-500">{{ step.text }}</p>
              </li>
            </ol>
          </div>
        </div>
      </section>

      <!-- ============ RANKING (liga) ============ -->
      <section class="bg-slate-900 px-4 sm:px-6 py-20 md:py-28 text-white">
        <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div>
            <span class="inline-flex rotate-2 rounded-xl bg-yellow-300 px-3 py-1 text-sm font-black text-slate-900">🏆 Clasificación</span>
            <h2 class="mt-5 text-4xl md:text-6xl font-black tracking-tight leading-[1.05]">
              Compite con tu uni.<br /><span class="text-emerald-400">Llega al top.</span>
            </h2>
            <p class="mt-5 max-w-md text-lg font-semibold text-slate-400">
              Cada misión completada suma XP para ti, tu universidad y tu carrera.
              Las empresas ven primero a quien va arriba.
            </p>
            <div class="mt-8 flex flex-wrap gap-3">
              <span class="badge bg-emerald-500/15 text-emerald-300">💎 Insignias</span>
              <span class="badge bg-sky-500/15 text-sky-300">📈 Niveles</span>
              <span class="badge bg-orange-500/15 text-orange-300">🔥 Rachas</span>
            </div>
          </div>

          <div class="rounded-[2rem] border-2 border-slate-700 bg-slate-800 p-3 shadow-[0_8px_0_#020617]">
            <div class="flex items-center gap-3 px-3 pt-2 pb-4">
              <span class="flex h-12 w-12 items-center justify-center rounded-2xl bg-emerald-500 text-2xl shadow-[0_4px_0_#047857]">💎</span>
              <div>
                <p class="text-xl font-black">Liga Esmeralda</p>
                <p class="text-sm font-bold text-slate-400">Temporada actual</p>
              </div>
            </div>

            <div class="grid grid-cols-3 gap-1 rounded-2xl bg-slate-900 p-1">
              <button
                v-for="tab in rankingTabs"
                :key="tab.key"
                type="button"
                :class="[
                  'truncate rounded-xl px-2 py-2.5 text-xs sm:text-sm font-black transition',
                  activeRanking === tab.key ? 'bg-emerald-500 text-white' : 'text-slate-400 hover:text-white'
                ]"
                @click="activeRanking = tab.key"
              >
                {{ tab.label }}
              </button>
            </div>

            <ol class="mt-3 space-y-2">
              <li
                v-for="(entry, i) in currentRanking"
                :key="entry.name"
                :class="['flex items-center gap-3 rounded-2xl px-3 py-3', i === 0 ? 'bg-emerald-500/15' : 'bg-slate-900/50']"
              >
                <span class="w-8 text-center text-2xl">{{ medals[i] }}</span>
                <span :class="['flex h-11 w-11 shrink-0 items-center justify-center rounded-full text-sm font-black text-white', avatarColors[i]]">
                  {{ initials(entry.name) }}
                </span>
                <div class="min-w-0 flex-1">
                  <p class="truncate font-black">{{ entry.name }}</p>
                  <p class="truncate text-sm font-bold text-slate-400">{{ entry.detail }}</p>
                </div>
                <span class="font-black text-emerald-300 tabular-nums">{{ entry.points.toLocaleString('es-MX') }} XP</span>
              </li>
            </ol>
          </div>
        </div>
      </section>

      <!-- ============ EMPRESAS + CIERRE ============ -->
      <section class="px-4 sm:px-6 py-20 md:py-28">
        <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div class="relative overflow-hidden rounded-[2rem] bg-emerald-500 p-8 md:p-12 text-white shadow-[0_8px_0_#047857]">
            <span class="absolute -right-4 -top-4 text-8xl rotate-12 opacity-90" aria-hidden="true">🚀</span>
            <h2 class="relative max-w-sm text-3xl md:text-4xl font-black leading-tight">¿List@ para tu primera misión?</h2>
            <p class="relative mt-3 max-w-sm text-lg font-semibold text-emerald-50">Crea tu perfil con tu correo universitario y empieza a postularte hoy.</p>
            <router-link :to="studentCta.to" class="btn-3d btn-white mt-8 inline-block px-7 py-3.5 text-lg">
              {{ studentCta.label }}
            </router-link>
          </div>

          <div class="relative overflow-hidden rounded-[2rem] bg-violet-500 p-8 md:p-12 text-white shadow-[0_8px_0_#5b21b6]">
            <span class="absolute -right-4 -top-4 text-8xl -rotate-12 opacity-90" aria-hidden="true">🤝</span>
            <h2 class="relative max-w-sm text-3xl md:text-4xl font-black leading-tight">¿Tienes una empresa?</h2>
            <p class="relative mt-3 max-w-sm text-lg font-semibold text-violet-100">Publica una vacante y encuentra estudiantes con ganas de demostrar lo que saben.</p>
            <router-link :to="isEmployer ? '/jobs/create' : '/empresas'" class="btn-3d btn-white mt-8 inline-block px-7 py-3.5 text-lg !text-violet-600">
              {{ isEmployer ? 'Publicar vacante' : 'Ver planes' }}
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
import DefaultLayout from '../layouts/DefaultLayout.vue'
import JobService from '../services/JobService'
import { useAuth } from '../composables/useAuth'

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
  isLoggedIn() ? { to: '/jobs', label: 'Explorar misiones' } : { to: '/register', label: 'Crear mi perfil' }
)

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

const initials = name => name.split(' ').map(w => w[0]).join('').slice(0, 2).toUpperCase()

const stats = computed(() => [
  { label: 'misiones abiertas', value: jobs.value.length, emoji: '🎯', box: 'border-emerald-200 bg-emerald-50', iconBox: 'bg-emerald-200' },
  { label: 'empresas contratando', value: new Set(jobs.value.map(companyName)).size, emoji: '🏢', box: 'border-sky-200 bg-sky-50', iconBox: 'bg-sky-200' },
  { label: 'desde tu casa', value: jobs.value.filter(j => j.isRemote).length, emoji: '🏠', box: 'border-yellow-200 bg-yellow-50', iconBox: 'bg-yellow-200' }
])

const heroJob = computed(() => {
  const job = jobs.value.find(j => j.highlighted) || jobs.value[0]
  if (!job) return { initial: 'U', title: 'Desarrollador Frontend', company: 'Startup', salary: '$15–20/hora', link: '/jobs' }
  return {
    initial: companyName(job)[0]?.toUpperCase() || '?',
    title: job.title,
    company: companyName(job),
    salary: formatSalary(job),
    link: `/jobs/${job._id}`
  }
})

// ---------- Ofertas ----------
const filters = {
  all: () => true,
  remote: j => j.isRemote,
  highlighted: j => j.highlighted,
  project: j => j.salaryRange?.type === 'proyecto'
}

const filterOptions = [
  { key: 'all', label: 'Todas' },
  { key: 'remote', label: '🏠 Remoto' },
  { key: 'highlighted', label: '⭐ Top' },
  { key: 'project', label: '📦 Proyecto' }
]

const featuredJobs = computed(() =>
  jobs.value
    .filter(filters[activeFilter.value])
    .sort((a, b) => Number(!!b.highlighted) - Number(!!a.highlighted))
    .slice(0, 6)
)

const avatarColors = ['bg-emerald-500', 'bg-sky-500', 'bg-violet-500', 'bg-orange-400', 'bg-pink-500', 'bg-yellow-400']

// ---------- Búsqueda ----------
const popularSearches = [
  { emoji: '💻', label: 'Vue' },
  { emoji: '🎨', label: 'Figma' },
  { emoji: '🐍', label: 'Python' },
  { emoji: '📣', label: 'Marketing' },
  { emoji: '🎬', label: 'Video' }
]

const chipColors = [
  'bg-emerald-100 text-emerald-700 shadow-[0_3px_0_#a7f3d0]',
  'bg-pink-100 text-pink-700 shadow-[0_3px_0_#fbcfe8]',
  'bg-sky-100 text-sky-700 shadow-[0_3px_0_#bae6fd]',
  'bg-yellow-100 text-yellow-800 shadow-[0_3px_0_#fde68a]',
  'bg-violet-100 text-violet-700 shadow-[0_3px_0_#ddd6fe]'
]

const goSearch = term => {
  const q = (term ?? heroQuery.value).trim()
  router.push({ path: '/jobs', query: q ? { q } : {} })
}

// ---------- Categorías ----------
const categories = [
  { name: 'Programación', emoji: '💻', query: 'desarrollo', tile: 'tile-emerald', keywords: ['desarrollo', 'developer', 'frontend', 'backend', 'vue', 'react', 'javascript', 'python', 'software'] },
  { name: 'Diseño', emoji: '🎨', query: 'diseño', tile: 'tile-pink', keywords: ['diseño', 'ux', 'ui', 'figma', 'design'] },
  { name: 'Marketing', emoji: '📣', query: 'marketing', tile: 'tile-orange', keywords: ['marketing', 'redes', 'social', 'seo', 'growth'] },
  { name: 'Datos e IA', emoji: '📊', query: 'datos', tile: 'tile-sky', keywords: ['datos', 'data', 'sql', 'análisis', 'analista', 'ia'] },
  { name: 'Video y foto', emoji: '🎬', query: 'video', tile: 'tile-violet', keywords: ['video', 'edición', 'foto', 'motion'] },
  { name: 'Redacción', emoji: '✍️', query: 'redacción', tile: 'tile-yellow', keywords: ['redacción', 'redactor', 'contenido', 'copy'] },
  { name: 'Negocios', emoji: '💼', query: 'negocios', tile: 'tile-sky', keywords: ['negocios', 'ventas', 'finanzas', 'business'] },
  { name: 'Tutorías', emoji: '📚', query: 'tutor', tile: 'tile-emerald', keywords: ['tutor', 'clases', 'asesoría', 'enseñanza'] },
  { name: 'Idiomas', emoji: '🌎', query: 'traducción', tile: 'tile-yellow', keywords: ['traducción', 'idiomas', 'inglés', 'english'] },
  { name: 'Ingeniería', emoji: '⚙️', query: 'ingeniería', tile: 'tile-violet', keywords: ['ingeniería', 'cad', 'mecánica', 'industrial'] }
]

const categoriesWithCount = computed(() =>
  categories.map(cat => ({
    ...cat,
    count: jobs.value.filter(job => {
      const text = [job.title, job.description, ...(job.tags || [])].join(' ').toLowerCase()
      return cat.keywords.some(k => text.includes(k))
    }).length
  }))
)

// ---------- Camino ----------
const steps = [
  { emoji: '📝', title: 'Crea tu perfil', xp: 100, text: 'Regístrate con tu correo universitario. Toma 5 minutos.', node: 'node-emerald' },
  { emoji: '🔍', title: 'Elige una misión', xp: 250, text: 'Filtra por área, pago o modalidad y postúlate.', node: 'node-sky' },
  { emoji: '🚀', title: 'Complétala', xp: 500, text: 'Trabaja con la empresa y recibe feedback real.', node: 'node-violet' },
  { emoji: '👑', title: 'Sube al top', xp: 1000, text: 'Gana insignias y destaca ante más empresas.', node: 'node-yellow' }
]

// ---------- Ranking ----------
const medals = ['🥇', '🥈', '🥉']

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
</script>

<style scoped>
/* Botones "3D" */
.btn-3d {
  border-radius: 1rem;
  font-weight: 900;
  text-align: center;
  transition: transform 0.1s, box-shadow 0.1s, filter 0.15s;
}
.btn-3d:hover { filter: brightness(1.05); }
.btn-3d:active { transform: translateY(4px); box-shadow: none; }

.btn-green { background: #10b981; color: #fff; box-shadow: 0 5px 0 #047857; }
.btn-white { background: #fff; color: #047857; border: 2px solid #e2e8f0; box-shadow: 0 5px 0 #e2e8f0; }

.chip {
  border-radius: 0.9rem;
  padding: 0.45rem 0.9rem;
  font-size: 0.9rem;
  font-weight: 800;
  transition: transform 0.1s, box-shadow 0.1s;
}
.chip:hover { transform: translateY(-2px); }
.chip:active { transform: translateY(2px); box-shadow: none; }

.sticker {
  border: 2px solid #1e293b;
  border-radius: 0.9rem;
  padding: 0.5rem 0.9rem;
  font-weight: 900;
  font-size: 0.95rem;
  white-space: nowrap;
  box-shadow: 3px 3px 0 #1e293b;
}

.card {
  border-radius: 1.5rem;
  border: 2px solid #e2e8f0;
  background: #fff;
  padding: 1.5rem;
  box-shadow: 0 5px 0 #e2e8f0;
  transition: transform 0.15s, box-shadow 0.15s, border-color 0.15s;
}
.card:hover { transform: translateY(-3px); border-color: #a7f3d0; box-shadow: 0 8px 0 #a7f3d0; }

.tile {
  display: block;
  border-radius: 1.5rem;
  border: 2px solid;
  padding: 1.25rem;
  transition: transform 0.12s, box-shadow 0.12s;
}
.tile:hover { transform: translateY(-3px); }
.tile:active { transform: translateY(3px); box-shadow: none; }
.tile-emerald { background: #ecfdf5; border-color: #a7f3d0; color: #065f46; box-shadow: 0 5px 0 #a7f3d0; }
.tile-pink    { background: #fdf2f8; border-color: #fbcfe8; color: #9d174d; box-shadow: 0 5px 0 #fbcfe8; }
.tile-orange  { background: #fff7ed; border-color: #fed7aa; color: #9a3412; box-shadow: 0 5px 0 #fed7aa; }
.tile-sky     { background: #f0f9ff; border-color: #bae6fd; color: #075985; box-shadow: 0 5px 0 #bae6fd; }
.tile-violet  { background: #f5f3ff; border-color: #ddd6fe; color: #5b21b6; box-shadow: 0 5px 0 #ddd6fe; }
.tile-yellow  { background: #fefce8; border-color: #fde68a; color: #854d0e; box-shadow: 0 5px 0 #fde68a; }

.level-node {
  position: relative;
  display: flex;
  height: 6rem;
  width: 6rem;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
}
.node-emerald { background: #10b981; box-shadow: 0 7px 0 #047857; }
.node-sky     { background: #38bdf8; box-shadow: 0 7px 0 #0369a1; }
.node-violet  { background: #8b5cf6; box-shadow: 0 7px 0 #5b21b6; }
.node-yellow  { background: #facc15; box-shadow: 0 7px 0 #a16207; }

.badge {
  border-radius: 0.9rem;
  padding: 0.5rem 0.9rem;
  font-weight: 900;
}

/* Animaciones */
.mascot { animation: bob 3s ease-in-out infinite; }
.wave { transform-origin: 156px 124px; animation: wave 2.4s ease-in-out infinite; }
.float-a { animation: float 4s ease-in-out infinite; }
.float-b { animation: float 5s ease-in-out infinite 0.6s; }
.float-c { animation: float 4.5s ease-in-out infinite 1.2s; }

@keyframes bob {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-8px); }
}
@keyframes wave {
  0%, 60%, 100% { transform: rotate(0); }
  70% { transform: rotate(-18deg); }
  80% { transform: rotate(8deg); }
  90% { transform: rotate(-12deg); }
}
@keyframes float {
  0%, 100% { translate: 0 0; }
  50% { translate: 0 -8px; }
}

@media (prefers-reduced-motion: reduce) {
  .mascot, .wave, .float-a, .float-b, .float-c { animation: none; }
}
</style>
