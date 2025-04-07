<template>
    <div class="min-h-screen flex flex-col">
        <!-- Navbar -->
        <nav class="bg-white border-b px-6 py-3 flex items-center justify-between">
            <!-- Logo -->
            <router-link to="/" class="flex items-center gap-2 font-bold text-green-700 text-xl">
                <span class="text-2xl">🎓</span>
                internships.gg
            </router-link>


            <!-- Middle nav -->
            <div class="hidden md:flex gap-6 text-sm font-medium text-gray-700">
                <router-link to="/jobs" class="hover:text-green-700">Explorar</router-link>
                <a href="#" class="hover:text-green-700">Categorías</a>
                <a href="#" class="hover:text-green-700">Cómo funciona</a>
                <a href="#" class="hover:text-green-700">Para empresas</a>
            </div>

            <!-- Right side -->
            <div class="flex items-center gap-3">
                <!-- Notificaciones -->
                <button class="relative p-2 rounded hover:bg-gray-100">
                    <span class="text-xl">🔔</span>
                    <span class="absolute top-1 right-1 w-2 h-2 bg-green-500 rounded-full"></span>
                </button>

                <!-- Botones -->
                <router-link to="/login" class="border px-4 py-2 rounded-md text-sm hover:bg-gray-50">
                    Iniciar sesión
                </router-link>
                <router-link to="/register"
                    class="bg-green-600 text-white px-4 py-2 rounded-md text-sm hover:bg-green-700">
                    Registrarse
                </router-link>
            </div>
        </nav>

        <!-- Main content -->
        <main class="flex-grow p-4">
            <slot />
        </main>

        <!-- Footer -->
        <footer class="bg-white border-t py-10 px-6 text-sm text-gray-600">
  <div class="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8">
    <!-- Branding -->
    <div>
      <div class="flex items-center gap-2 font-bold text-green-700 text-xl mb-2">
        🎓 internships.gg
      </div>
      <p class="text-gray-500">Conectando talento universitario con oportunidades laborales desde 2023.</p>
    </div>

    <!-- Estudiantes -->
    <div>
      <h4 class="font-semibold text-gray-800 mb-2">Para estudiantes</h4>
      <ul class="space-y-1">
        <li><a href="#" class="hover:underline">Cómo funciona</a></li>
        <li><a href="#" class="hover:underline">Crear perfil</a></li>
        <li><a href="#" class="hover:underline">Buscar oportunidades</a></li>
        <li><a href="#" class="hover:underline">Recursos</a></li>
      </ul>
    </div>

    <!-- Empresas -->
    <div>
      <h4 class="font-semibold text-gray-800 mb-2">Para empresas</h4>
      <ul class="space-y-1">
        <li><a href="#" class="hover:underline">Publicar ofertas</a></li>
        <li><a href="#" class="hover:underline">Buscar talento</a></li>
        <li><a href="#" class="hover:underline">Planes y precios</a></li>
        <li><a href="#" class="hover:underline">Casos de éxito</a></li>
      </ul>
    </div>

    <!-- Contacto -->
    <div>
      <h4 class="font-semibold text-gray-800 mb-2">Contacto</h4>
      <ul class="space-y-1">
        <li><a href="#" class="hover:underline">Soporte</a></li>
        <li><a href="#" class="hover:underline">Prensa</a></li>
        <li><a href="#" class="hover:underline">Términos y condiciones</a></li>
        <li><a href="#" class="hover:underline">Privacidad</a></li>
      </ul>
    </div>
  </div>

  <!-- Línea inferior -->
  <div class="text-center text-xs text-gray-400 mt-10 border-t pt-4">
    © {{ new Date().getFullYear() }} internships.gg. Todos los derechos reservados.
  </div>
</footer>

    </div>
</template>