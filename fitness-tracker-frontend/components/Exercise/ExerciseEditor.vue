<template>
  <UCard>
    <template #header>
      <div class="row flex justify-between">
        <h2 class="text-lg font-semibold">{{ model?.name ?? "" }}</h2>
        <UButton icon="i-lucide-trash" color="error" variant="ghost" @click="$emit('remove')" />
      </div>
    </template>
    <template v-for="set in model?.sets">
      <div class="grid grid-cols-3 gap-4">
        <SetInputRow :step="1" label="reps" v-model="set.reps" />
        <SetInputRow :step="0.25" label="kg" v-model="set.weight" />
        <DurationInputRow label="min" v-model="set.duration" />
      </div>
    </template>
  </UCard>
</template>

<script lang="ts" setup>
import type { Exercise } from "~/types/Exercise/Exercise";
import SetInputRow from "./NumberInputRow.vue";
import DurationInputRow from "./DurationInputRow.vue";

const model = defineModel<Exercise>();
defineEmits(["remove"]);
</script>
