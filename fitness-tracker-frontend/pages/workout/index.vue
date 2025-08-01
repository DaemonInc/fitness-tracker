<template>
  <PageTitle>{{ $t("workout.list.title") }}</PageTitle>
</template>

<script lang="ts" setup>
useHead({
  title: $t("workout.list.title"),
  meta: [
    {
      name: "description",
      content: $t("workout.list.description"),
    },
  ],
});
</script>
