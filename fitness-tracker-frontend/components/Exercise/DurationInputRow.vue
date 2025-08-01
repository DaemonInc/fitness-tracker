<template>
  <div class="row">
    <UInputNumber
      class="w-24"
      :controls="false"
      :min="0"
      :step="1"
      v-model="minutes"
      :decrement-disabled="true"
      :format-options="{
        maximumFractionDigits: 0,
      }"
    >
      <template #increment><div /></template>
      <template #decrement><div /></template>
    </UInputNumber>
    <span class="px-1">:</span>
    <UInputNumber
      class="w-24"
      :controls="false"
      :min="0"
      :step="1"
      v-model="seconds"
      :format-options="{
        minimumIntegerDigits: 2,
        maximumFractionDigits: 0,
      }"
    >
      <template #increment><div /></template>
      <template #decrement><div /></template
    ></UInputNumber>
    {{ label }}
  </div>
</template>

<script lang="ts" setup>
const model = defineModel<number | null>({
  type: Number,
});
defineProps<{
  label: string;
}>();

const minutes = computed({
  get: () => Math.floor((model.value ?? 0) / 60),
  set: (value: number) => {
    const currentSeconds = (model.value ?? 0) % 60;
    model.value = value * 60 + currentSeconds;
  },
});
const seconds = computed({
  get: () => (model.value ?? 0) % 60,
  set: (value: number) => {
    console.log("Setting seconds:", value);
    const currentMinutes = Math.floor((model.value ?? 0) / 60);
    model.value = currentMinutes * 60 + value;
  },
});
</script>
