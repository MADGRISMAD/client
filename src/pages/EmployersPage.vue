<template>
    <DefaultLayout>
      <section class="min-h-screen bg-gradient-to-b from-white to-gray-50 pb-24 flex flex-col items-center px-4">
        <!-- HERO -->
        <div class="text-center pt-12 pb-8 space-y-3 max-w-4xl">
          <h1 class="text-3xl sm:text-4xl font-bold tracking-tight">
            Encuentra el <span class="text-emerald-600">talento</span> que necesitas
          </h1>
          <p class="text-base text-gray-500 max-w-2xl mx-auto">
            Conecta con estudiantes talentosos y potencia tus proyectos con el plan perfecto para tu empresa.
          </p>
        </div>
  
        <!-- PLANES -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-5xl w-full">
          <!-- Básico -->
          <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm hover:shadow-md transition">
            <h2 class="text-lg font-semibold text-center mb-2">Básico</h2>
            <p class="text-3xl font-bold text-center text-emerald-600 mb-1">Gratis</p>
            <p class="text-sm text-center text-gray-500 mb-4">Para startups y pequeños equipos</p>
            <ul class="space-y-2 text-sm mb-6">
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>2 ofertas activas</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Perfil básico</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Hasta 20 aplicaciones/mes</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Soporte por email</span>
              </li>
              <li class="flex items-center gap-2 text-gray-400">
                <span>✗</span>
                <span>Filtros avanzados</span>
              </li>
              <li class="flex items-center gap-2 text-gray-400">
                <span>✗</span>
                <span>Ofertas destacadas</span>
              </li>
              <li class="flex items-center gap-2 text-gray-400">
                <span>✗</span>
                <span>Analíticas</span>
              </li>
            </ul>
            <button class="w-full border border-emerald-600 text-emerald-600 py-2 rounded-lg font-medium hover:bg-emerald-50 transition">
              Comenzar gratis
            </button>
          </div>
  
          <!-- Profesional -->
          <div class="bg-white border-2 border-emerald-200 rounded-lg p-6 shadow-sm relative">
            <span class="absolute -top-3 left-1/2 transform -translate-x-1/2 bg-emerald-600 text-white text-xs font-medium px-3 py-1 rounded-full">
              Más popular
            </span>
            <h2 class="text-lg font-semibold text-center mb-2">Profesional</h2>
            <p class="text-3xl font-bold text-center text-emerald-600 mb-1">$99 <span class="text-sm font-normal">/mes</span></p>
            <p class="text-sm text-center text-gray-500 mb-4">Para empresas en crecimiento</p>
            <ul class="space-y-2 text-sm mb-6">
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>10 ofertas activas</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Perfil destacado</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Aplicaciones ilimitadas</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Soporte prioritario</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Filtros avanzados</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>3 ofertas destacadas/mes</span>
              </li>
              <li class="flex items-center gap-2 text-gray-400">
                <span>✗</span>
                <span>Analíticas</span>
              </li>
            </ul>
            <button class="w-full bg-emerald-600 text-white py-2 rounded-lg font-medium hover:bg-emerald-700 transition">
              Prueba gratuita
            </button>
          </div>
  
          <!-- Empresarial -->
          <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm hover:shadow-md transition">
            <h2 class="text-lg font-semibold text-center mb-2">Empresarial</h2>
            <p class="text-3xl font-bold text-center text-emerald-600 mb-1">$249 <span class="text-sm font-normal">/mes</span></p>
            <p class="text-sm text-center text-gray-500 mb-4">Para grandes empresas</p>
            <ul class="space-y-2 text-sm mb-6">
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Ofertas ilimitadas</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Perfil premium</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Aplicaciones ilimitadas</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Soporte 24/7</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Filtros avanzados</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>10 ofertas destacadas/mes</span>
              </li>
              <li class="flex items-center gap-2">
                <span class="text-emerald-600">✓</span>
                <span>Analíticas avanzadas</span>
              </li>
            </ul>
            <button class="w-full border border-emerald-600 text-emerald-600 py-2 rounded-lg font-medium hover:bg-emerald-50 transition">
              Contactar ventas
            </button>
          </div>
        </div>
  
        <!-- COMPARATIVA -->
        <div class="mt-16 max-w-5xl w-full">
          <h2 class="text-xl font-semibold text-center mb-6">Comparativa de características</h2>
          <div class="overflow-x-auto">
            <table class="min-w-full text-sm border border-gray-200 text-center bg-white rounded-lg overflow-hidden">
              <thead class="bg-gray-50">
                <tr>
                  <th class="p-3 border-b text-left">Funcionalidad</th>
                  <th class="p-3 border-b">Básico</th>
                  <th class="p-3 border-b bg-emerald-50">Profesional</th>
                  <th class="p-3 border-b">Empresarial</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td class="p-3 border-b text-left">Ofertas activas</td>
                  <td class="p-3 border-b">2</td>
                  <td class="p-3 border-b bg-emerald-50">10</td>
                  <td class="p-3 border-b">Ilimitadas</td>
                </tr>
                <tr>
                  <td class="p-3 border-b text-left">Aplicaciones</td>
                  <td class="p-3 border-b">20/mes</td>
                  <td class="p-3 border-b bg-emerald-50">Ilimitadas</td>
                  <td class="p-3 border-b">Ilimitadas</td>
                </tr>
                <tr>
                  <td class="p-3 border-b text-left">Soporte</td>
                  <td class="p-3 border-b">Email</td>
                  <td class="p-3 border-b bg-emerald-50">Prioritario</td>
                  <td class="p-3 border-b">24/7</td>
                </tr>
                <tr>
                  <td class="p-3 border-b text-left">Ofertas destacadas</td>
                  <td class="p-3 border-b text-gray-400">No</td>
                  <td class="p-3 border-b bg-emerald-50">3/mes</td>
                  <td class="p-3 border-b">10/mes</td>
                </tr>
                <tr>
                  <td class="p-3 text-left">Analíticas</td>
                  <td class="p-3 text-gray-400">No</td>
                  <td class="p-3 bg-emerald-50 text-gray-400">No</td>
                  <td class="p-3">Avanzadas</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- SOLUCIÓN PERSONALIZADA -->
        <div class="mt-16 max-w-5xl w-full">
          <div class="bg-gray-900 text-white rounded-lg p-6">
            <div class="flex flex-col md:flex-row gap-6 items-center">
              <div class="md:w-2/3">
                <h2 class="text-xl font-semibold mb-3">¿Necesitas una solución personalizada?</h2>
                <p class="text-gray-300 mb-4 text-sm">
                  Para empresas con necesidades específicas, ofrecemos planes personalizados que incluyen características
                  adicionales como integración con tu ATS, marca personalizada y más.
                </p>
                <ul class="space-y-3">
                  <li class="flex items-start gap-2">
                    <span class="text-emerald-400">✓</span>
                    <span class="text-sm">Integración con tu sistema de gestión de talento</span>
                  </li>
                  <li class="flex items-start gap-2">
                    <span class="text-emerald-400">✓</span>
                    <span class="text-sm">Portal de reclutamiento personalizado con tu marca</span>
                  </li>
                  <li class="flex items-start gap-2">
                    <span class="text-emerald-400">✓</span>
                    <span class="text-sm">Eventos de reclutamiento exclusivos en universidades</span>
                  </li>
                  <li class="flex items-start gap-2">
                    <span class="text-emerald-400">✓</span>
                    <span class="text-sm">Gestión de programas de pasantías y prácticas profesionales</span>
                  </li>
                </ul>
              </div>
              <div class="md:w-1/3">
                <div class="bg-white text-gray-900 rounded-lg p-4">
                  <h3 class="text-lg font-semibold text-center mb-2">Plan Empresarial Plus</h3>
                  <p class="text-gray-600 text-xs text-center mb-4">
                    Contacta con nuestro equipo de ventas para obtener un presupuesto personalizado según tus
                    necesidades específicas.
                  </p>
                  <button class="w-full bg-emerald-600 text-white py-2 rounded-lg font-medium hover:bg-emerald-700 transition">
                    Solicitar información
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
  
        <!-- TESTIMONIOS -->
        <div class="mt-16 max-w-5xl w-full">
          <h2 class="text-xl font-semibold text-center mb-6">Lo que dicen nuestros clientes</h2>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
              <p class="text-sm text-gray-600 mb-4">"Gracias a esta plataforma, encontramos el talento perfecto para nuestro equipo. ¡Altamente recomendado!"</p>
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-full bg-emerald-100 flex items-center justify-center font-semibold text-emerald-600">
                  JP
                </div>
                <div>
                  <p class="font-medium text-sm text-gray-900">Juan Pérez</p>
                  <p class="text-xs text-gray-500">CEO, Startup XYZ</p>
                </div>
              </div>
            </div>
            <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
              <p class="text-sm text-gray-600 mb-4">"La calidad de los candidatos es impresionante. Hemos ahorrado tiempo y recursos en el proceso de selección."</p>
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-full bg-emerald-100 flex items-center justify-center font-semibold text-emerald-600">
                  ML
                </div>
                <div>
                  <p class="font-medium text-sm text-gray-900">María López</p>
                  <p class="text-xs text-gray-500">Gerente de RRHH, Empresa ABC</p>
                </div>
              </div>
            </div>
            <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
              <p class="text-sm text-gray-600 mb-4">"Una herramienta indispensable para conectar con estudiantes talentosos. ¡Excelente servicio!"</p>
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-full bg-emerald-100 flex items-center justify-center font-semibold text-emerald-600">
                  CG
                </div>
                <div>
                  <p class="font-medium text-sm text-gray-900">Carlos Gómez</p>
                  <p class="text-xs text-gray-500">Director de Proyectos, Compañía DEF</p>
                </div>
              </div>
            </div>
          </div>
        </div>
  
        <!-- CTA -->
        <div class="mt-16 max-w-5xl w-full">
          <div class="bg-emerald-600 text-white rounded-lg p-6 text-center">
            <h2 class="text-xl font-semibold mb-3">¿Listo para encontrar el mejor talento universitario?</h2>
            <p class="text-sm mb-4 max-w-2xl mx-auto">
              Únete a cientos de empresas que ya están aprovechando el potencial de estudiantes para impulsar sus proyectos.
            </p>
            <router-link
              to="/register"
              class="inline-block bg-white text-emerald-600 px-4 py-2 rounded-lg font-medium text-sm hover:bg-emerald-50 transition"
            >
              Empezar ahora
            </router-link>
          </div>
        </div>
      </section>
    </DefaultLayout>
  </template>
  
  <script setup>
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  </script>
