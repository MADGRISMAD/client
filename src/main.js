import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import '@fontsource-variable/geist'
import '@fontsource-variable/geist-mono'
import './assets/tailwind.css'

const app = createApp(App).use(router)

/**
 * v-reveal: el elemento aparece al entrar en pantalla (una sola vez). Con IntersectionObserver, sin escuchar el scroll.
 * Si la persona pidió reducir el movimiento, no se añade nada y el contenido se ve de inmediato.
 */
app.directive('reveal', {
  mounted(el, binding) {
    if (typeof IntersectionObserver === 'undefined' || matchMedia('(prefers-reduced-motion: reduce)').matches) return
    if (binding.value != null) el.style.setProperty('--i', binding.value)
    el.classList.add('reveal')
    const io = new IntersectionObserver(
      ([entry]) => {
        if (!entry.isIntersecting) return
        el.classList.add('is-in')
        io.disconnect()
      },
      { rootMargin: '0px 0px -8% 0px' }
    )
    io.observe(el)
    el._revealIo = io
  },
  unmounted(el) {
    el._revealIo?.disconnect()
  }
})

app.mount('#app')
