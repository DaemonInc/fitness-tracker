<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>

<script setup lang="ts">
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
});
</script>
