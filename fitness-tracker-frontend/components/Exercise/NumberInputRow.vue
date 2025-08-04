<template>
  <div class="row">
    <UInputNumber :min="0" :step="step" v-model="model" class="mr-1">
      <template #increment><div /></template>
      <template #decrement><div /></template>
    </UInputNumber>
    {{ label }}
  </div>
</template>

<script lang="ts" setup>
const model = defineModel<number | null>({
  type: Number,
  default: null,
});

defineProps<{
  step: number;
  label: string;
}>();
</script>
