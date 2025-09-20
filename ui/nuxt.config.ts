// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2025-07-15',
    devtools: {enabled: true},
    colorMode: {
        preference: 'light',
        fallback: 'light'
    },
    ui: {
        colorMode: false
    },
    modules: [
      '@nuxt/eslint',
      '@nuxt/image',
      '@nuxt/ui',
      '@nuxt/fonts',
      '@nuxtjs/color-mode',
    ]
    ,
})