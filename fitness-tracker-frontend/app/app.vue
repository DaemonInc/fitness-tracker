<template>
  <div>
    <UApp :locale="locales[locale]">
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </UApp>
  </div>
</template>

<script setup lang="ts">
import * as locales from "@nuxt/ui/locale";

const { locale } = useI18n();

const lang = computed(() => locales[locale.value].code);
const dir = computed(() => locales[locale.value].dir);

useSeoMeta({
  title: "Fitness Tracker",
  ogTitle: "Fitness Tracker",
  description:
    "Track your fitness activities and progress with our comprehensive fitness tracker app.",
  ogDescription:
    "Track your fitness activities and progress with our comprehensive fitness tracker app.",
});
useHead({
  titleTemplate: (title) => (title ? `${title} - Fitness Tracker` : "Fitness Tracker"),
  htmlAttrs: {
    lang,
    dir,
  },
});
</script>
