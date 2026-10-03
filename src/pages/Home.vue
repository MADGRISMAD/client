<template>
  <DefaultLayout>
    <div class="bg-white text-gray-900 overflow-x-clip">
      <!-- ============ HERO ============ -->
      <section class="relative isolate bg-gray-950 text-white overflow-hidden">
        <!-- Fondo: glows + grid -->
        <div class="pointer-events-none absolute inset-0 -z-10">
          <div class="absolute -top-40 -left-32 h-[34rem] w-[34rem] rounded-full bg-emerald-500/25 blur-3xl glow-a"></div>
          <div class="absolute top-20 -right-40 h-[30rem] w-[30rem] rounded-full bg-teal-400/20 blur-3xl glow-b"></div>
          <div class="absolute bottom-[-12rem] left-1/3 h-[26rem] w-[26rem] rounded-full bg-lime-400/10 blur-3xl"></div>
          <div class="absolute inset-0 hero-grid"></div>
        </div>

        <div class="max-w-7xl mx-auto px-4 sm:px-6 pt-16 pb-20 md:pt-24 md:pb-28">
          <div class="grid lg:grid-cols-12 gap-12 lg:gap-8 items-center">
            <!-- Copy -->
            <div class="lg:col-span-7">
              <router-link
                to="/jobs"
                class="group inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 pl-1.5 pr-3 py-1 text-xs sm:text-sm text-gray-300 backdrop-blur hover:border-emerald-400/40 transition-colors"
              >
                <span class="rounded-full bg-emerald-500 px-2 py-0.5 text-[11px] font-semibold text-gray-950">Nuevo</span>
                Ofertas remotas y por proyecto<span class="hidden sm:inline"> cada semana</span>
                <span class="text-emerald-400 transition-transform group-hover:translate-x-0.5">→</span>
              </router-link>

              <h1 class="mt-6 text-4xl sm:text-5xl lg:text-6xl xl:text-7xl font-extrabold tracking-tight leading-[1.05]">
                Tu primera gran
                <span class="relative whitespace-nowrap">
                  <span class="bg-gradient-to-r from-emerald-300 via-teal-200 to-lime-200 bg-clip-text text-transparent">oportunidad</span>
                  <svg class="absolute -bottom-2 left-0 w-full h-3 text-emerald-400/70" viewBox="0 0 300 12" fill="none" preserveAspectRatio="none" aria-hidden="true">
                    <path d="M2 9C60 3 140 1 298 6" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
                  </svg>
                </span>
                empieza aquí.
              </h1>

              <p class="mt-6 max-w-xl text-lg sm:text-xl text-gray-400 leading-relaxed">
                Trabajos freelance, remotos y por proyecto pensados para estudiantes universitarios.
                Gana experiencia real, sube de nivel y construye tu portafolio mientras estudias.
              </p>

              <!-- Buscador -->
              <form
                @submit.prevent="goSearch()"
                class="mt-8 flex flex-col sm:flex-row gap-2 rounded-2xl border border-white/10 bg-white/[0.06] p-2 backdrop-blur-xl shadow-2xl shadow-emerald-950/40 focus-within:border-emerald-400/50 transition-colors"
              >
                <label class="relative flex-1">
                  <span class="sr-only">Buscar ofertas</span>
                  <svg class="absolute left-4 top-1/2 -translate-y-1/2 h-5 w-5 text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                    <circle cx="11" cy="11" r="7" /><path d="m20 20-3.5-3.5" stroke-linecap="round" />
                  </svg>
                  <input
                    v-model="heroQuery"
                    type="text"
                    placeholder="Puesto, habilidad o etiqueta…"
                    class="w-full bg-transparent pl-12 pr-4 py-3.5 text-base text-white placeholder:text-gray-500 focus:outline-none"
                  />
                </label>
                <button
                  type="submit"
                  class="inline-flex items-center justify-center gap-2 rounded-xl bg-emerald-500 px-6 py-3.5 font-semibold text-gray-950 hover:bg-emerald-400 active:scale-[0.98] transition"
                >
                  Buscar ofertas
                </button>
              </form>

              <div class="mt-4 flex flex-wrap items-center gap-2 text-sm">
                <span class="text-gray-500">Populares:</span>
                <button
                  v-for="term in popularSearches"
                  :key="term"
                  type="button"
                  @click="goSearch(term)"
                  class="rounded-full border border-white/10 px-3 py-1 text-gray-300 hover:border-emerald-400/50 hover:text-white transition-colors"
                >
                  {{ term }}
                </button>
              </div>

              <div class="mt-10 flex flex-col sm:flex-row gap-3">
                <router-link
                  :to="primaryCta.to"
                  class="inline-flex items-center justify-center gap-2 rounded-xl bg-white px-6 py-3.5 font-semibold text-gray-950 hover:bg-gray-100 transition"
                >
                  {{ primaryCta.label }}
                  <span aria-hidden="true">→</span>
                </router-link>
                <router-link
                  to="/empresas"
                  class="inline-flex items-center justify-center gap-2 rounded-xl border border-white/15 px-6 py-3.5 font-semibold text-white hover:bg-white/5 transition"
                >
                  Soy empresa
                </router-link>
              </div>
            </div>

            <!-- Visual -->
            <div class="lg:col-span-5 relative hidden sm:block">
              <div class="relative mx-auto max-w-md py-16">
                <!-- Tarjeta principal -->
                <div class="float-slow relative rounded-3xl border border-white/10 bg-white/[0.07] p-6 backdrop-blur-xl shadow-2xl shadow-black/40">
                  <div class="flex items-start justify-between">
                    <div class="flex items-center gap-4">
                      <div class="h-14 w-14 rounded-2xl bg-gradient-to-br from-emerald-400 to-teal-500 flex items-center justify-center text-xl font-bold text-gray-950">
                        {{ heroJob.initial }}
                      </div>
                      <div>
                        <p class="text-xs font-medium uppercase tracking-wider text-emerald-300">Destacada</p>
                        <h3 class="text-lg font-semibold text-white leading-snug">{{ heroJob.title }}</h3>
                        <p class="text-sm text-gray-400">{{ heroJob.company }}</p>
                      </div>
                    </div>
                    <span v-if="heroJob.remote" class="rounded-full bg-emerald-400/15 px-2.5 py-1 text-xs font-medium text-emerald-300">Remoto</span>
                  </div>
                  <p class="mt-5 text-sm text-gray-300 line-clamp-2">{{ heroJob.description }}</p>
                  <div class="mt-5 flex flex-wrap gap-2">
                    <span
                      v-for="tag in heroJob.tags"
                      :key="tag"
                      class="rounded-lg bg-white/5 border border-white/10 px-2.5 py-1 text-xs text-gray-300"
                    >#{{ tag }}</span>
                  </div>
                  <div class="mt-6 flex items-center justify-between border-t border-white/10 pt-5">
                    <span class="text-sm font-semibold text-white">{{ heroJob.salary }}</span>
                    <router-link
                      :to="heroJob.link"
                      class="rounded-lg bg-emerald-500 px-4 py-2 text-sm font-semibold text-gray-950 hover:bg-emerald-400 transition"
                    >
                      Aplicar
                    </router-link>
                  </div>
                </div>

                <!-- Notificación -->
                <div class="float-fast absolute -top-16 right-0 lg:-right-8 flex items-center gap-3 rounded-2xl border border-white/10 bg-gray-900/90 px-4 py-3 shadow-xl backdrop-blur">
                  <span class="relative flex h-9 w-9 items-center justify-center rounded-full bg-emerald-500/20 text-emerald-300">
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true"><path d="m5 12 5 5L20 7" stroke-linecap="round" stroke-linejoin="round" /></svg>
                    <span class="absolute -top-0.5 -right-0.5 h-2.5 w-2.5 rounded-full bg-emerald-400 ring-2 ring-gray-900"></span>
                  </span>
                  <div>
                    <p class="text-sm font-semibold text-white">¡Postulación aceptada!</p>
                    <p class="text-xs text-gray-400">Hace unos segundos</p>
                  </div>
                </div>

                <!-- XP -->
                <div class="float-mid absolute -bottom-20 -left-4 lg:-left-10 w-60 rounded-2xl border border-white/10 bg-gray-900/90 p-4 shadow-xl backdrop-blur">
                  <div class="flex items-center justify-between text-sm">
                    <span class="font-semibold text-white">Nivel 7</span>
                    <span class="text-emerald-300 font-medium">+120 XP</span>
                  </div>
                  <div class="mt-3 h-2 rounded-full bg-white/10 overflow-hidden">
                    <div class="xp-bar h-full rounded-full bg-gradient-to-r from-emerald-400 to-lime-300"></div>
                  </div>
                  <p class="mt-2 text-xs text-gray-400">2,450 / 3,000 XP para el siguiente nivel</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Value props -->
        <div class="border-t border-white/10 bg-white/[0.02]">
          <div class="max-w-7xl mx-auto px-4 sm:px-6 py-6 grid grid-cols-2 md:grid-cols-4 gap-6">
            <div v-for="prop in valueProps" :key="prop.title" class="flex items-center gap-3">
              <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-emerald-400/10 text-emerald-300" v-html="prop.icon"></span>
              <div>
                <p class="text-sm font-semibold text-white">{{ prop.title }}</p>
                <p class="text-xs text-gray-400">{{ prop.text }}</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- ============ UNIVERSIDADES (marquee) ============ -->
      <section class="py-10 border-b border-gray-100">
        <p class="text-center text-xs font-semibold uppercase tracking-[0.2em] text-gray-400">
          Estudiantes de las mejores universidades ya están aquí
        </p>
        <div class="marquee mt-6">
          <div class="marquee-track">
            <span
              v-for="(uni, i) in [...universities, ...universities]"
              :key="uni + i"
              class="mx-8 text-xl sm:text-2xl font-bold tracking-tight text-gray-300 whitespace-nowrap"
            >{{ uni }}</span>
          </div>
        </div>
      </section>

      <!-- ============ CATEGORÍAS ============ -->
      <section id="categorias" class="py-20 md:py-28 px-4 sm:px-6">
        <div class="max-w-7xl mx-auto">
          <div v-reveal class="reveal flex flex-col md:flex-row md:items-end justify-between gap-6 mb-12">
            <div>
              <p class="text-sm font-semibold text-emerald-600">Explora por área</p>
              <h2 class="mt-2 text-3xl md:text-5xl font-extrabold tracking-tight">Encuentra lo que te mueve</h2>
            </div>
            <p class="max-w-md text-gray-500">
              Desde código hasta campañas: elige tu especialidad y descubre proyectos hechos a tu medida.
            </p>
          </div>

          <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <router-link
              v-for="(cat, i) in categoriesWithCount"
              :key="cat.name"
              :to="{ path: '/jobs', query: { q: cat.query } }"
              v-reveal
              :style="{ '--d': `${i * 60}ms` }"
              :class="[
                'reveal group relative overflow-hidden rounded-3xl border border-gray-200 p-6 transition-all duration-300 hover:-translate-y-1 hover:shadow-xl hover:shadow-emerald-900/5 hover:border-emerald-200',
                i === 0 ? 'col-span-2 lg:row-span-2 bg-gradient-to-br from-emerald-50 via-white to-white' : 'bg-white'
              ]"
            >
              <span
                :class="[
                  'flex items-center justify-center rounded-2xl text-emerald-700 bg-emerald-100 transition-transform duration-300 group-hover:scale-110 group-hover:-rotate-6',
                  i === 0 ? 'h-16 w-16' : 'h-12 w-12'
                ]"
                v-html="cat.icon"
              ></span>
              <h3 :class="['mt-6 font-bold tracking-tight', i === 0 ? 'text-2xl md:text-3xl' : 'text-lg']">{{ cat.name }}</h3>
              <p :class="['mt-1 text-gray-500', i === 0 ? 'text-base max-w-sm' : 'text-sm']">{{ cat.description }}</p>
              <div class="mt-6 flex items-center justify-between text-sm">
                <span class="font-medium text-gray-700">
                  {{ cat.count ? `${cat.count} ${cat.count === 1 ? 'oferta' : 'ofertas'}` : 'Ver ofertas' }}
                </span>
                <span class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 text-gray-500 transition-colors group-hover:bg-emerald-600 group-hover:text-white">→</span>
              </div>
              <div v-if="i === 0" class="pointer-events-none absolute -right-16 -bottom-16 h-56 w-56 rounded-full bg-emerald-200/40 blur-2xl"></div>
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ OFERTAS DESTACADAS ============ -->
      <section id="ofertas" class="py-20 md:py-28 px-4 sm:px-6 bg-gray-50">
        <div class="max-w-7xl mx-auto">
          <div v-reveal class="reveal flex flex-col lg:flex-row lg:items-end justify-between gap-6 mb-10">
            <div>
              <p class="text-sm font-semibold text-emerald-600">Recién publicadas</p>
              <h2 class="mt-2 text-3xl md:text-5xl font-extrabold tracking-tight">Ofertas destacadas</h2>
              <p v-if="!loading && jobs.length" class="mt-3 text-gray-500">
                {{ jobs.length }} ofertas activas · {{ companiesCount }} empresas · {{ remoteCount }} remotas
              </p>
            </div>

            <div class="flex flex-wrap gap-2">
              <button
                v-for="f in filterOptions"
                :key="f.key"
                type="button"
                @click="activeFilter = f.key"
                :class="[
                  'rounded-full px-4 py-2 text-sm font-medium transition-colors',
                  activeFilter === f.key
                    ? 'bg-gray-900 text-white'
                    : 'bg-white border border-gray-200 text-gray-600 hover:border-gray-300 hover:text-gray-900'
                ]"
              >
                {{ f.label }}
              </button>
            </div>
          </div>

          <!-- Loading -->
          <div v-if="loading" class="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <div v-for="n in 6" :key="n" class="rounded-3xl bg-white border border-gray-200 p-6 animate-pulse">
              <div class="flex items-center gap-4">
                <div class="h-12 w-12 rounded-2xl bg-gray-200"></div>
                <div class="flex-1 space-y-2">
                  <div class="h-4 w-2/3 rounded bg-gray-200"></div>
                  <div class="h-3 w-1/3 rounded bg-gray-100"></div>
                </div>
              </div>
              <div class="mt-6 space-y-2">
                <div class="h-3 rounded bg-gray-100"></div>
                <div class="h-3 w-5/6 rounded bg-gray-100"></div>
              </div>
              <div class="mt-6 h-10 rounded-xl bg-gray-100"></div>
            </div>
          </div>

          <!-- Error -->
          <div v-else-if="error" class="rounded-3xl border border-dashed border-gray-300 bg-white p-12 text-center">
            <p class="text-lg font-semibold">No pudimos cargar las ofertas</p>
            <p class="mt-2 text-gray-500">Revisa tu conexión e inténtalo de nuevo en unos momentos.</p>
            <button
              type="button"
              @click="loadJobs"
              class="mt-6 rounded-xl bg-gray-900 px-5 py-2.5 text-sm font-semibold text-white hover:bg-gray-800 transition"
            >
              Reintentar
            </button>
          </div>

          <!-- Vacío -->
          <div v-else-if="!featuredJobs.length" class="rounded-3xl border border-dashed border-gray-300 bg-white p-12 text-center">
            <p class="text-lg font-semibold">No hay ofertas con este filtro por ahora</p>
            <p class="mt-2 text-gray-500">Prueba con otro filtro o explora todas las ofertas disponibles.</p>
            <router-link to="/jobs" class="mt-6 inline-flex rounded-xl bg-gray-900 px-5 py-2.5 text-sm font-semibold text-white hover:bg-gray-800 transition">
              Ver todas las ofertas
            </router-link>
          </div>

          <!-- Grid -->
          <div v-else class="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <router-link
              v-for="job in featuredJobs"
              :key="job._id"
              :to="`/jobs/${job._id}`"
              :class="[
                'group relative flex flex-col rounded-3xl border bg-white p-6 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-emerald-900/10',
                job.highlighted ? 'border-emerald-300 ring-1 ring-emerald-200' : 'border-gray-200 hover:border-emerald-200'
              ]"
            >
              <div class="flex items-start justify-between gap-3">
                <div class="flex items-center gap-4 min-w-0">
                  <div class="h-12 w-12 shrink-0 rounded-2xl bg-gradient-to-br from-emerald-100 to-teal-100 flex items-center justify-center text-lg font-bold text-emerald-700">
                    {{ companyName(job)[0]?.toUpperCase() || '?' }}
                  </div>
                  <div class="min-w-0">
                    <h3 class="font-semibold text-gray-900 leading-snug truncate group-hover:text-emerald-700 transition-colors">{{ job.title }}</h3>
                    <p class="text-sm text-gray-500 truncate">{{ companyName(job) }}</p>
                  </div>
                </div>
                <span v-if="job.highlighted" class="shrink-0 rounded-full bg-amber-100 px-2.5 py-1 text-xs font-semibold text-amber-700">★ Top</span>
              </div>

              <p class="mt-5 text-sm text-gray-600 line-clamp-2">{{ job.description }}</p>

              <div class="mt-5 flex flex-wrap gap-2">
                <span v-if="job.isRemote" class="rounded-lg bg-emerald-50 px-2.5 py-1 text-xs font-medium text-emerald-700">Remoto</span>
                <span
                  v-for="tag in (job.tags || []).slice(0, 3)"
                  :key="tag"
                  class="rounded-lg bg-gray-100 px-2.5 py-1 text-xs text-gray-600"
                >#{{ tag }}</span>
              </div>

              <div class="mt-auto pt-6">
                <div class="flex items-center justify-between border-t border-gray-100 pt-5 text-sm">
                  <span class="font-semibold text-gray-900">{{ formatSalary(job) }}</span>
                  <span class="text-gray-500">{{ job.duration || 'Flexible' }}</span>
                </div>
              </div>
            </router-link>
          </div>

          <div v-if="!loading && !error && jobs.length" class="mt-12 text-center">
            <router-link
              to="/jobs"
              class="inline-flex items-center gap-2 rounded-xl border border-gray-300 bg-white px-6 py-3 font-semibold text-gray-900 hover:border-gray-900 transition-colors"
            >
              Explorar todas las ofertas
              <span aria-hidden="true">→</span>
            </router-link>
          </div>
        </div>
      </section>

      <!-- ============ CÓMO FUNCIONA ============ -->
      <section id="como-funciona" class="py-20 md:py-28 px-4 sm:px-6">
        <div class="max-w-7xl mx-auto">
          <div v-reveal class="reveal text-center max-w-2xl mx-auto mb-16">
            <p class="text-sm font-semibold text-emerald-600">Así de simple</p>
            <h2 class="mt-2 text-3xl md:text-5xl font-extrabold tracking-tight">De estudiante a profesional en tres niveles</h2>
            <p class="mt-4 text-lg text-gray-500">Cada paso suma experiencia. Literalmente.</p>
          </div>

          <div class="relative grid md:grid-cols-3 gap-6">
            <div class="hidden md:block absolute top-12 left-[16%] right-[16%] h-px bg-gradient-to-r from-emerald-200 via-emerald-400 to-emerald-200"></div>
            <div
              v-for="(step, i) in steps"
              :key="step.title"
              v-reveal
              :style="{ '--d': `${i * 120}ms` }"
              class="reveal relative text-center px-4"
            >
              <div class="relative mx-auto flex h-24 w-24 items-center justify-center rounded-3xl bg-white border border-gray-200 shadow-lg shadow-emerald-900/5">
                <span class="text-emerald-600" v-html="step.icon"></span>
                <span class="absolute -top-3 -right-3 flex h-8 w-8 items-center justify-center rounded-full bg-gray-900 text-xs font-bold text-white ring-4 ring-white">
                  {{ i + 1 }}
                </span>
              </div>
              <p class="mt-6 text-xs font-semibold uppercase tracking-widest text-emerald-600">Nivel {{ i + 1 }} · +{{ step.xp }} XP</p>
              <h3 class="mt-2 text-xl font-bold">{{ step.title }}</h3>
              <p class="mt-2 text-gray-500 max-w-xs mx-auto">{{ step.text }}</p>
            </div>
          </div>
        </div>
      </section>

      <!-- ============ RANKING ============ -->
      <section class="relative isolate overflow-hidden bg-gray-950 text-white py-20 md:py-28 px-4 sm:px-6">
        <div class="pointer-events-none absolute inset-0 -z-10">
          <div class="absolute -top-32 right-0 h-[28rem] w-[28rem] rounded-full bg-emerald-500/20 blur-3xl"></div>
          <div class="absolute inset-0 hero-grid opacity-60"></div>
        </div>

        <div class="max-w-7xl mx-auto grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div v-reveal class="reveal">
            <p class="text-sm font-semibold text-emerald-400">Ranking de talento</p>
            <h2 class="mt-2 text-3xl md:text-5xl font-extrabold tracking-tight">Compite, destaca y deja tu marca</h2>
            <p class="mt-5 text-lg text-gray-400 max-w-lg">
              Cada proyecto completado suma puntos para ti, tu universidad y tu especialidad.
              Las empresas ven primero a quienes lideran la tabla.
            </p>
            <ul class="mt-8 space-y-4">
              <li v-for="perk in rankingPerks" :key="perk" class="flex items-start gap-3 text-gray-300">
                <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-emerald-500/20 text-emerald-300">
                  <svg class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" aria-hidden="true"><path d="m5 12 5 5L20 7" stroke-linecap="round" stroke-linejoin="round" /></svg>
                </span>
                {{ perk }}
              </li>
            </ul>
          </div>

          <div v-reveal class="reveal rounded-3xl border border-white/10 bg-white/[0.04] p-2 backdrop-blur-xl shadow-2xl">
            <div class="grid grid-cols-3 gap-1 rounded-2xl bg-white/5 p-1">
              <button
                v-for="tab in rankingTabs"
                :key="tab.key"
                type="button"
                @click="activeRanking = tab.key"
                :class="[
                  'rounded-xl px-2 sm:px-3 py-2.5 text-xs sm:text-sm font-semibold transition-colors truncate',
                  activeRanking === tab.key ? 'bg-white text-gray-950' : 'text-gray-400 hover:text-white'
                ]"
              >
                {{ tab.label }}
              </button>
            </div>

            <ul class="p-4 sm:p-6 space-y-5">
              <li v-for="(entry, i) in currentRanking" :key="entry.name" class="flex items-center gap-4">
                <span
                  :class="[
                    'flex h-10 w-10 shrink-0 items-center justify-center rounded-xl text-sm font-bold',
                    i === 0 ? 'bg-amber-300 text-amber-950' : i === 1 ? 'bg-gray-300 text-gray-900' : 'bg-orange-300 text-orange-950'
                  ]"
                >
                  {{ i + 1 }}
                </span>
                <div class="flex-1 min-w-0">
                  <div class="flex items-baseline justify-between gap-3">
                    <p class="font-semibold truncate">{{ entry.name }}</p>
                    <p class="text-sm font-semibold text-emerald-300 tabular-nums">{{ entry.points.toLocaleString('es-MX') }} pts</p>
                  </div>
                  <p class="text-xs text-gray-400">{{ entry.detail }}</p>
                  <div class="mt-2 h-1.5 rounded-full bg-white/10 overflow-hidden">
                    <div
                      class="h-full rounded-full bg-gradient-to-r from-emerald-400 to-lime-300 transition-all duration-700"
                      :style="{ width: `${(entry.points / currentRanking[0].points) * 100}%` }"
                    ></div>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </section>

      <!-- ============ CTA FINAL ============ -->
      <section class="py-20 md:py-28 px-4 sm:px-6">
        <div class="max-w-7xl mx-auto grid md:grid-cols-2 gap-5">
          <div v-reveal class="reveal relative overflow-hidden rounded-[2rem] bg-gradient-to-br from-emerald-500 via-emerald-600 to-teal-700 p-8 md:p-12 text-white">
            <div class="pointer-events-none absolute -right-20 -top-20 h-64 w-64 rounded-full bg-white/10 blur-2xl"></div>
            <p class="text-sm font-semibold uppercase tracking-widest text-emerald-100">Para estudiantes</p>
            <h2 class="mt-3 text-3xl md:text-4xl font-extrabold tracking-tight">Empieza a ganar experiencia hoy</h2>
            <p class="mt-4 text-emerald-50/90 max-w-md">Crea tu perfil gratis con tu correo universitario y postúlate en minutos.</p>
            <router-link
              :to="studentCta.to"
              class="mt-8 inline-flex items-center gap-2 rounded-xl bg-white px-6 py-3.5 font-semibold text-emerald-700 hover:bg-emerald-50 transition"
            >
              {{ studentCta.label }} <span aria-hidden="true">→</span>
            </router-link>
          </div>

          <div v-reveal class="reveal relative overflow-hidden rounded-[2rem] bg-gray-950 p-8 md:p-12 text-white" style="--d: 120ms">
            <div class="pointer-events-none absolute inset-0 hero-grid opacity-50"></div>
            <div class="relative">
              <p class="text-sm font-semibold uppercase tracking-widest text-gray-400">Para empresas</p>
              <h2 class="mt-3 text-3xl md:text-4xl font-extrabold tracking-tight">Contrata talento joven y motivado</h2>
              <p class="mt-4 text-gray-400 max-w-md">Publica una vacante y recibe postulaciones de estudiantes listos para aportar desde el día uno.</p>
              <router-link
                :to="isEmployer ? '/jobs/create' : '/empresas'"
                class="mt-8 inline-flex items-center gap-2 rounded-xl bg-emerald-500 px-6 py-3.5 font-semibold text-gray-950 hover:bg-emerald-400 transition"
              >
                {{ isEmployer ? 'Publicar vacante' : 'Ver planes' }} <span aria-hidden="true">→</span>
              </router-link>
            </div>
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

const primaryCta = computed(() => {
  if (!isLoggedIn()) return { to: '/register', label: 'Crear cuenta gratis' }
  if (isEmployer.value) return { to: '/jobs/create', label: 'Publicar vacante' }
  return { to: '/jobs', label: 'Explorar ofertas' }
})

const studentCta = computed(() =>
  isLoggedIn() ? { to: '/jobs', label: 'Explorar ofertas' } : { to: '/register', label: 'Crear mi perfil' }
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
  const range = s.min != null && s.max != null ? `$${s.min} – $${s.max}` : `$${s.min ?? s.max}`
  return s.type ? `${range} / ${s.type}` : range
}

const companiesCount = computed(() => new Set(jobs.value.map(companyName)).size)
const remoteCount = computed(() => jobs.value.filter(j => j.isRemote).length)

const filterOptions = [
  { key: 'all', label: 'Todas' },
  { key: 'remote', label: 'Remoto' },
  { key: 'highlighted', label: 'Destacadas' },
  { key: 'project', label: 'Por proyecto' }
]

const featuredJobs = computed(() => {
  let list = [...jobs.value]
  if (activeFilter.value === 'remote') list = list.filter(j => j.isRemote)
  if (activeFilter.value === 'highlighted') list = list.filter(j => j.highlighted)
  if (activeFilter.value === 'project') list = list.filter(j => j.salaryRange?.type === 'proyecto')
  list.sort((a, b) => Number(!!b.highlighted) - Number(!!a.highlighted))
  return list.slice(0, 6)
})

const heroJob = computed(() => {
  const job = jobs.value.find(j => j.highlighted) || jobs.value[0]
  if (!job) {
    return {
      initial: 'U',
      title: 'Desarrollador Frontend',
      company: 'Startup en crecimiento',
      description: 'Buscamos a alguien con ganas de aprender Vue.js para un proyecto de 6 meses.',
      tags: ['Vue', 'JavaScript', 'Frontend'],
      salary: '$15 – $20 / hora',
      remote: true,
      link: '/jobs'
    }
  }
  return {
    initial: companyName(job)[0]?.toUpperCase() || '?',
    title: job.title,
    company: companyName(job),
    description: job.description,
    tags: (job.tags || []).slice(0, 3),
    salary: formatSalary(job),
    remote: job.isRemote,
    link: `/jobs/${job._id}`
  }
})

// ---------- Búsqueda ----------
const popularSearches = ['Vue', 'Diseño UX', 'Marketing', 'Python', 'Datos']

const goSearch = term => {
  const q = (term ?? heroQuery.value).trim()
  router.push({ path: '/jobs', query: q ? { q } : {} })
}

// ---------- Contenido ----------
const icon = (path, size = 'h-6 w-6') =>
  `<svg class="${size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${path}</svg>`

const valueProps = [
  { title: 'Perfiles verificados', text: 'Solo correos universitarios', icon: icon('<path d="M12 3 4 6v6c0 4.5 3.4 8.3 8 9 4.6-.7 8-4.5 8-9V6l-8-3Z"/><path d="m9 12 2 2 4-4"/>') },
  { title: 'Pago transparente', text: 'Tarifa visible en cada oferta', icon: icon('<rect x="3" y="6" width="18" height="13" rx="2"/><path d="M3 10h18M7 15h3"/>') },
  { title: 'Trabajo flexible', text: 'Remoto y por proyecto', icon: icon('<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18"/>') },
  { title: 'Avisos al instante', text: 'Sigue cada postulación', icon: icon('<path d="M6 8a6 6 0 1 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/><path d="M10 21a2 2 0 0 0 4 0"/>') }
]

const universities = ['UNAM', 'IPN', 'Tec de Monterrey', 'UDG', 'UANL', 'ITAM', 'UAM', 'BUAP', 'UDLAP', 'Ibero']

const categories = [
  { name: 'Desarrollo de software', query: 'desarrollo', description: 'Frontend, backend, apps móviles y todo lo que se construye con código.', keywords: ['desarrollo', 'developer', 'frontend', 'backend', 'vue', 'react', 'javascript', 'python', 'software'], icon: icon('<path d="m8 9-4 3 4 3M16 9l4 3-4 3M14 5l-4 14"/>') },
  { name: 'Diseño UX/UI', query: 'diseño', description: 'Interfaces, prototipos y branding.', keywords: ['diseño', 'ux', 'ui', 'figma', 'design'], icon: icon('<path d="M12 19c-4 0-8-3-8-7a8 8 0 0 1 16 0c0 2-1.5 3-3 3h-2a2 2 0 0 0-1 3.7c.5.3.4 1.3-2 .3Z"/><circle cx="8.5" cy="10.5" r="1"/><circle cx="12" cy="7.5" r="1"/><circle cx="15.5" cy="10.5" r="1"/>') },
  { name: 'Marketing digital', query: 'marketing', description: 'Redes, contenido y growth.', keywords: ['marketing', 'redes', 'social', 'seo', 'growth'], icon: icon('<path d="M3 11v2a1 1 0 0 0 1 1h2l5 4V6L6 10H4a1 1 0 0 0-1 1Z"/><path d="M15 9a4 4 0 0 1 0 6M18 6a8 8 0 0 1 0 12"/>') },
  { name: 'Datos e IA', query: 'datos', description: 'Análisis, ML y visualización.', keywords: ['datos', 'data', 'ia', 'ai', 'machine', 'sql', 'análisis'], icon: icon('<path d="M4 20V10M10 20V4M16 20v-7M22 20H2"/>') },
  { name: 'Contenido', query: 'contenido', description: 'Redacción, video y edición.', keywords: ['contenido', 'redacción', 'video', 'copy', 'edición'], icon: icon('<path d="M4 20h4L19 9a2.8 2.8 0 0 0-4-4L4 16v4Z"/><path d="m13.5 6.5 4 4"/>') },
  { name: 'Negocios', query: 'negocios', description: 'Ventas, finanzas y operaciones.', keywords: ['negocios', 'ventas', 'finanzas', 'administración', 'business'], icon: icon('<rect x="3" y="7" width="18" height="13" rx="2"/><path d="M8 7V5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M3 13h18"/>') },
  { name: 'Tutorías', query: 'tutor', description: 'Asesorías y clases en línea.', keywords: ['tutor', 'clases', 'asesoría', 'educación', 'enseñanza'], icon: icon('<path d="m2 9 10-5 10 5-10 5L2 9Z"/><path d="M6 11v5c0 1 2.7 3 6 3s6-2 6-3v-5M22 9v6"/>') },
  { name: 'Idiomas', query: 'traducción', description: 'Traducción e interpretación.', keywords: ['traducción', 'idiomas', 'inglés', 'english', 'translation'], icon: icon('<path d="M4 5h8M8 3v2c0 4-2 7-5 9M6 9c1 2.5 3 4 6 5"/><path d="m13 21 4-9 4 9M14.5 18h5"/>') },
  { name: 'Ingeniería', query: 'ingeniería', description: 'CAD, procesos y manufactura.', keywords: ['ingeniería', 'cad', 'mecánica', 'industrial'], icon: icon('<circle cx="12" cy="12" r="3"/><path d="M12 2v3M12 19v3M4.2 4.2l2.1 2.1M17.7 17.7l2.1 2.1M2 12h3M19 12h3M4.2 19.8l2.1-2.1M17.7 6.3l2.1-2.1"/>') }
]

const categoriesWithCount = computed(() =>
  categories.map(cat => {
    const count = jobs.value.filter(job => {
      const haystack = [job.title, job.description, ...(job.tags || [])].join(' ').toLowerCase()
      return cat.keywords.some(k => haystack.includes(k))
    }).length
    return { ...cat, count }
  })
)

const steps = [
  { title: 'Crea tu perfil', xp: 100, text: 'Regístrate con tu correo universitario y muestra tus habilidades.', icon: icon('<circle cx="12" cy="8" r="4"/><path d="M4 21a8 8 0 0 1 16 0"/>', 'h-9 w-9') },
  { title: 'Encuentra tu match', xp: 250, text: 'Filtra por modalidad, área o etiquetas y postúlate en un clic.', icon: icon('<circle cx="11" cy="11" r="7"/><path d="m20 20-3.5-3.5"/>', 'h-9 w-9') },
  { title: 'Gana experiencia', xp: 500, text: 'Trabaja con empresas reales, recibe feedback y sube de nivel.', icon: icon('<path d="M5 15c-1.5 1.3-2 5-2 5s3.7-.5 5-2c.7-.8.7-2.1-.1-2.9a2.2 2.2 0 0 0-2.9-.1Z"/><path d="m12 15-3-3a22 22 0 0 1 2-4A13 13 0 0 1 22 2c0 2.7-.8 7.5-6 11a22 22 0 0 1-4 2Z"/>', 'h-9 w-9') }
]

// ---------- Ranking ----------
const rankingTabs = [
  { key: 'students', label: 'Estudiantes' },
  { key: 'universities', label: 'Universidades' },
  { key: 'specialties', label: 'Especialidades' }
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

const rankingPerks = [
  'Insignias y niveles visibles en tu perfil',
  'Mayor visibilidad ante empresas que contratan',
  'Compite junto a tu universidad por el primer lugar'
]

// ---------- Animación al hacer scroll ----------
const vReveal = {
  mounted(el) {
    if (typeof IntersectionObserver === 'undefined') {
      el.classList.add('is-visible')
      return
    }
    const observer = new IntersectionObserver(
      entries => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            el.classList.add('is-visible')
            observer.disconnect()
          }
        })
      },
      { threshold: 0.15 }
    )
    observer.observe(el)
    el._revealObserver = observer
  },
  unmounted(el) {
    el._revealObserver?.disconnect()
  }
}
</script>

<style scoped>
.hero-grid {
  background-image:
    linear-gradient(to right, rgb(255 255 255 / 0.05) 1px, transparent 1px),
    linear-gradient(to bottom, rgb(255 255 255 / 0.05) 1px, transparent 1px);
  background-size: 56px 56px;
  mask-image: radial-gradient(ellipse 70% 60% at 50% 30%, #000 40%, transparent 100%);
}

.glow-a { animation: drift 18s ease-in-out infinite; }
.glow-b { animation: drift 22s ease-in-out infinite reverse; }

@keyframes drift {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(40px, 30px) scale(1.08); }
}

.float-slow { animation: float 7s ease-in-out infinite; }
.float-mid { animation: float 6s ease-in-out infinite 0.8s; }
.float-fast { animation: float 5s ease-in-out infinite 1.6s; }

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

.xp-bar {
  width: 82%;
  animation: fill 1.6s cubic-bezier(0.22, 1, 0.36, 1) both 0.6s;
}

@keyframes fill {
  from { width: 0; }
}

.marquee {
  overflow: hidden;
  mask-image: linear-gradient(to right, transparent, #000 12%, #000 88%, transparent);
}

.marquee-track {
  display: flex;
  width: max-content;
  animation: scroll 40s linear infinite;
}

.marquee:hover .marquee-track { animation-play-state: paused; }

@keyframes scroll {
  to { transform: translateX(-50%); }
}

.reveal {
  opacity: 0;
  transform: translateY(24px);
  transition-property: opacity, transform, translate, box-shadow, border-color, color;
  transition-duration: 700ms, 700ms, 300ms, 300ms, 300ms, 300ms;
  transition-delay: var(--d, 0ms), var(--d, 0ms), 0ms, 0ms, 0ms, 0ms;
  transition-timing-function: cubic-bezier(0.22, 1, 0.36, 1);
}

.reveal.is-visible {
  opacity: 1;
  transform: none;
}

@media (prefers-reduced-motion: reduce) {
  .glow-a, .glow-b, .float-slow, .float-mid, .float-fast, .xp-bar, .marquee-track {
    animation: none;
  }
  .reveal {
    opacity: 1;
    transform: none;
    transition: none;
  }
}
</style>
