<template>
    <DefaultLayout>
      <section class="min-h-screen bg-gradient-to-b from-white to-gray-50 pb-24 flex flex-col items-center px-4">
        <!-- HERO -->
        <div class="text-center pt-16 pb-12 space-y-4 max-w-4xl">
          <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight leading-tight">
            Planes y <span class="text-emerald-600">precios</span>
          </h1>
          <p class="text-lg text-gray-500 max-w-2xl mx-auto">
            Encuentra el plan perfecto para conectar con talento universitario y potenciar tus proyectos.
          </p>
        </div>
  
        <!-- PLANES -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl w-full">
          <!-- Básico -->
          <div class="bg-white border rounded-xl p-8 shadow-sm hover:shadow-md transition transform hover:-translate-y-1">
            <h2 class="text-xl font-bold text-center mb-2">Básico</h2>
            <p class="text-4xl font-extrabold text-center text-emerald-600 mb-1">Gratis</p>
            <p class="text-sm text-center text-gray-400 mb-6">Para startups y pequeños equipos</p>
            <ul class="space-y-3 text-sm mb-6">
              <li>✔️ 2 ofertas activas</li>
              <li>✔️ Perfil básico</li>
              <li>✔️ Hasta 20 aplicaciones/mes</li>
              <li>✔️ Soporte por email</li>
              <li class="text-gray-400">❌ Filtros avanzados</li>
              <li class="text-gray-400">❌ Ofertas destacadas</li>
              <li class="text-gray-400">❌ Analíticas</li>
            </ul>
            <button class="w-full border border-emerald-600 text-emerald-600 py-2.5 rounded-md font-medium hover:bg-emerald-50 transition">
              Comenzar gratis
            </button>
          </div>
  
          <!-- Profesional -->
          <div class="bg-white border-2 border-emerald-200 rounded-xl p-8 shadow-md relative hover:shadow-lg transition transform hover:-translate-y-1">
            <span class="absolute -top-3 left-1/2 transform -translate-x-1/2 bg-emerald-600 text-white text-xs font-medium px-3 py-1 rounded-full shadow">
              Más popular
            </span>
            <h2 class="text-xl font-bold text-center mb-2">Profesional</h2>
            <p class="text-4xl font-extrabold text-center text-emerald-600 mb-1">$99 <span class="text-base font-normal">/mes</span></p>
            <p class="text-sm text-center text-gray-400 mb-6">Para empresas en crecimiento</p>
            <ul class="space-y-3 text-sm mb-6">
              <li>✔️ 10 ofertas activas</li>
              <li>✔️ Perfil destacado</li>
              <li>✔️ Aplicaciones ilimitadas</li>
              <li>✔️ Soporte prioritario</li>
              <li>✔️ Filtros avanzados</li>
              <li>✔️ 3 ofertas destacadas/mes</li>
              <li class="text-gray-400">❌ Analíticas</li>
            </ul>
            <button class="w-full bg-emerald-600 text-white py-2.5 rounded-md font-medium hover:bg-emerald-700 transition">
              Prueba gratuita
            </button>
          </div>
  
          <!-- Empresarial -->
          <div class="bg-white border rounded-xl p-8 shadow-sm hover:shadow-md transition transform hover:-translate-y-1">
            <h2 class="text-xl font-bold text-center mb-2">Empresarial</h2>
            <p class="text-4xl font-extrabold text-center text-emerald-600 mb-1">$249 <span class="text-base font-normal">/mes</span></p>
            <p class="text-sm text-center text-gray-400 mb-6">Para grandes empresas</p>
            <ul class="space-y-3 text-sm mb-6">
              <li>✔️ Ofertas ilimitadas</li>
              <li>✔️ Perfil premium</li>
              <li>✔️ Aplicaciones ilimitadas</li>
              <li>✔️ Soporte 24/7</li>
              <li>✔️ Filtros avanzados</li>
              <li>✔️ 10 ofertas destacadas/mes</li>
              <li>✔️ Analíticas avanzadas</li>
            </ul>
            <button class="w-full border border-emerald-600 text-emerald-600 py-2.5 rounded-md font-medium hover:bg-emerald-50 transition">
              Contactar ventas
            </button>
          </div>
        </div>
  
        <!-- COMPARATIVA -->
        <div class="mt-20 max-w-6xl w-full px-2 py-10">
          <h2 class="text-2xl sm:text-3xl font-bold text-center mb-8">Comparativa de características</h2>
          <div class="overflow-x-auto">
            <table class="min-w-full text-sm border border-gray-200 text-center bg-white rounded-xl overflow-hidden shadow-sm">
              <thead class="bg-gray-50">
                <tr>
                  <th class="p-4 border-b text-left">Funcionalidad</th>
                  <th class="p-4 border-b">Básico</th>
                  <th class="p-4 border-b bg-emerald-50">Profesional</th>
                  <th class="p-4 border-b">Empresarial</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td class="p-4 border-b text-left">Ofertas activas</td>
                  <td class="p-4 border-b">2</td>
                  <td class="p-4 border-b bg-emerald-50">10</td>
                  <td class="p-4 border-b">Ilimitadas</td>
                </tr>
                <tr>
                  <td class="p-4 border-b text-left">Aplicaciones</td>
                  <td class="p-4 border-b">20/mes</td>
                  <td class="p-4 border-b bg-emerald-50">Ilimitadas</td>
                  <td class="p-4 border-b">Ilimitadas</td>
                </tr>
                <tr>
                  <td class="p-4 border-b text-left">Soporte</td>
                  <td class="p-4 border-b">Email</td>
                  <td class="p-4 border-b bg-emerald-50">Prioritario</td>
                  <td class="p-4 border-b">24/7</td>
                </tr>
                <tr>
                  <td class="p-4 border-b text-left">Ofertas destacadas</td>
                  <td class="p-4 border-b text-gray-400">No</td>
                  <td class="p-4 border-b bg-emerald-50">3/mes</td>
                  <td class="p-4 border-b">10/mes</td>
                </tr>
                <tr>
                  <td class="p-4 text-left">Analíticas</td>
                  <td class="p-4 text-gray-400">No</td>
                  <td class="p-4 bg-emerald-50 text-gray-400">No</td>
                  <td class="p-4">Avanzadas</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div class="bg-gray-900 text-white rounded-xl p-8 mb-16 shadow-lg max-w-7xl">
          <div class="flex flex-col md:flex-row gap-8 items-center">
            <div class="md:w-2/3">
              <h2 class="text-2xl font-bold mb-4">¿Necesitas una solución personalizada?</h2>
              <p class="text-gray-300 mb-6 leading-relaxed">
            Para empresas con necesidades específicas, ofrecemos planes personalizados que incluyen características
            adicionales como integración con tu ATS, marca personalizada, eventos de reclutamiento exclusivos y más.
              </p>
              <ul class="space-y-4">
            <li class="flex items-start gap-3">
              <svg class="h-6 w-6 text-emerald-400 flex-shrink-0" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>Integración con tu sistema de gestión de talento</span>
            </li>
            <li class="flex items-start gap-3">
              <svg class="h-6 w-6 text-emerald-400 flex-shrink-0" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>Portal de reclutamiento personalizado con tu marca</span>
            </li>
            <li class="flex items-start gap-3">
              <svg class="h-6 w-6 text-emerald-400 flex-shrink-0" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>Eventos de reclutamiento exclusivos en universidades</span>
            </li>
            <li class="flex items-start gap-3">
              <svg class="h-6 w-6 text-emerald-400 flex-shrink-0" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <span>Gestión de programas de pasantías y prácticas profesionales</span>
            </li>
              </ul>
            </div>
            <div class="md:w-1/3">
              <div class="bg-white text-gray-900 rounded-lg shadow-md p-6">
            <h3 class="text-xl font-semibold text-center mb-4">Plan Empresarial Plus</h3>
            <p class="text-gray-600 text-sm text-center mb-6">
              Contacta con nuestro equipo de ventas para obtener un presupuesto personalizado según tus
              necesidades específicas.
            </p>
            <button class="w-full bg-emerald-600 text-white py-2.5 rounded-md font-medium hover:bg-emerald-700 transition">
              Solicitar información
            </button>
              </div>
            </div>
          </div>
        </div>
        
  
        <!-- TESTIMONIOS -->
        <div class="mt-20 max-w-6xl w-full px-4">
          <h2 class="text-2xl sm:text-3xl font-bold text-center mb-12">Lo que dicen nuestros clientes</h2>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div class="bg-gradient-to-b from-white to-gray-50 border rounded-xl p-8 shadow-lg hover:shadow-xl transition transform hover:-translate-y-1">
              <p class="text-sm text-gray-600 italic mb-6">"Gracias a esta plataforma, encontramos el talento perfecto para nuestro equipo. ¡Altamente recomendado!"</p>
              <div class="flex items-center space-x-4">
            <img src="https://via.placeholder.com/50" alt="Cliente" class="w-14 h-14 rounded-full border-2 border-emerald-600">
            <div>
              <p class="font-bold text-base text-gray-800">Juan Pérez</p>
              <p class="text-sm text-gray-500">CEO, Startup XYZ</p>
            </div>
              </div>
            </div>
            <div class="bg-gradient-to-b from-white to-gray-50 border rounded-xl p-8 shadow-lg hover:shadow-xl transition transform hover:-translate-y-1">
              <p class="text-sm text-gray-600 italic mb-6">"La calidad de los candidatos es impresionante. Hemos ahorrado tiempo y recursos en el proceso de selección."</p>
              <div class="flex items-center space-x-4">
            <img src="https://via.placeholder.com/50" alt="Cliente" class="w-14 h-14 rounded-full border-2 border-emerald-600">
            <div>
              <p class="font-bold text-base text-gray-800">María López</p>
              <p class="text-sm text-gray-500">Gerente de Recursos Humanos, Empresa ABC</p>
            </div>
              </div>
            </div>
            <div class="bg-gradient-to-b from-white to-gray-50 border rounded-xl p-8 shadow-lg hover:shadow-xl transition transform hover:-translate-y-1">
              <p class="text-sm text-gray-600 italic mb-6">"Una herramienta indispensable para conectar con estudiantes talentosos. ¡Excelente servicio!"</p>
              <div class="flex items-center space-x-4">
            <img src="https://via.placeholder.com/50" alt="Cliente" class="w-14 h-14 rounded-full border-2 border-emerald-600">
            <div>
              <p class="font-bold text-base text-gray-800">Carlos Gómez</p>
              <p class="text-sm text-gray-500">Director de Proyectos, Compañía DEF</p>
            </div>
              </div>
            </div>
          </div>
        </div>
  
        <!-- CTA -->
        <div class="bg-emerald-600 text-white mt-20 py-12 px-6 rounded-xl shadow max-w-4xl text-center">
          <h2 class="text-2xl font-bold mb-4">¿Listo para encontrar el mejor talento universitario?</h2>
          <p class="text-lg mb-6 max-w-2xl mx-auto">
            Únete a cientos de empresas que ya están aprovechando el potencial de estudiantes para impulsar sus proyectos.
          </p>
          <router-link
            to="/register"
            class="bg-white text-emerald-600 px-6 py-3 rounded-md font-semibold text-sm hover:bg-emerald-50 transition"
          >
            Empezar ahora
          </router-link>
        </div>
      </section>
    </DefaultLayout>
  </template>
  

<script setup>
import DefaultLayout from '../layouts/DefaultLayout.vue'
</script>
