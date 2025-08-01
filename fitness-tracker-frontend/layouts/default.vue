<template>
  <UNavigationMenu :items="items" class="w-full justify-center" />
  <div class="mx-auto max-w-5xl items-center px-4 py-8">
    <slot />
  </div>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const items = ref<NavigationMenuItem[]>([
  { label: "Home", to: "/" },
  { label: "Workouts", to: "/workout" },
]);
</script>
