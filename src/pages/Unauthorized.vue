<template>
    <DefaultLayout>
      <div class="flex flex-col items-center justify-center min-h-[60vh] text-center px-6">
        <h1 class="text-4xl font-bold text-red-600 mb-4">🚫 Acceso no autorizado</h1>
        <p class="text-gray-600 mb-6">
          No tienes permisos para acceder a esta página. Si crees que esto es un error, por favor contacta al administrador.
        </p>
        <router-link
          to="/"
          class="inline-block bg-green-600 text-white px-5 py-2 rounded-md hover:bg-green-700 transition"
        >
          Volver al inicio
        </router-link>
      </div>
    </DefaultLayout>
  </template>
  
  <script setup>
  import DefaultLayout from '../layouts/DefaultLayout.vue'
  </script>
  