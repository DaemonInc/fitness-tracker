<template>
  <div class="row">
    <UInputNumber
      class="w-2/5"
      :min="0"
      :step="1"
      v-model="minutes"
      :format-options="{
        maximumFractionDigits: 0,
      }"
    />
    <span class="px-1">:</span>
    <UInputNumber
      class="w-2/5 mr-1"
      :min="0"
      :step="1"
      v-model="seconds"
      :format-options="{
        minimumIntegerDigits: 2,
        maximumFractionDigits: 0,
      }"
    />
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

<style scoped>
::-webkit-inner-spin-button {
  appearance: none;
}
</style>
