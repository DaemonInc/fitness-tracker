// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },

  typescript: {
    strict: true,
    typeCheck: true,
  },

  app: {
    head: {
      htmlAttrs: {
        lang: "en",
      },
      link: [
        {
          rel: "image",
          type: "image/x-icon",
          href: "/favicon.ico",
        },
      ],
    },
  },

  css: ["~/assets/css/main.css"],

  ui: {},

  image: {
    screens: {
      xs: 320,
      sm: 640,
      md: 768,
      lg: 1024,
      xl: 1280,
      xxl: 1536,
      "2xl": 1536,
    },
  },

  i18n: {
    defaultLocale: "en",
    langDir: "locales",
    locales: [
      {
        code: "en",
        name: "English",
        file: "en.json",
      },
      {
        code: "nl",
        name: "Nederlands",
        file: "nl.json",
      },
    ],
  },

  modules: ["@nuxt/image", "@nuxtjs/i18n", "@nuxt/ui"],
});
