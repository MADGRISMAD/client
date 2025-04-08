import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/Home.vue'
import Login from '../pages/Login.vue'
import Register from '../pages/Register.vue'
import Jobs from '../pages/Jobs.vue'
import AuthService from '../services/AuthService'

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/login', name: 'Login', component: Login },
  { path: '/register', name: 'Register', component: Register },
  { path: '/jobs', name: 'Jobs', component: Jobs },

  // Ruta dinámica: detalle de oferta
  {
    path: '/jobs/:id',
    name: 'JobDetail',
    component: () => import('../pages/JobDetail.vue')
  },

  // Nueva ruta: Crear oferta (solo employer)
  {
    path: '/jobs/create',
    name: 'CreateJob',
    component: () => import('../pages/CreateJob.vue'),
    meta: { requiresAuth: true, onlyEmployer: true }
  },

  // Ruta protegida general
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('../pages/Profile.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 🔐 Protección de rutas privadas y por rol
router.beforeEach((to, from, next) => {
  const isLoggedIn = AuthService.isLoggedIn()
  const user = AuthService.getUser() // debes tener este método

  if (to.meta.requiresAuth && !isLoggedIn) {
    return next('/login')
  }

  if (to.meta.onlyEmployer && user?.role !== 'employer') {
    return next('/unauthorized') // puedes crear una vista para esto
  }

  next()
})

export default router
