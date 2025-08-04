<template>
  <template v-for="(exercise, index) in modelValue" :key="exercise.uuid">
    <ExerciseEditor class="mb-4" :model-value="exercise" @remove="() => removeExercise(index)" />
  </template>
  <UButton
    label="Add Exercise"
    @click="
      addExercise({
        uuid: uuid(),
        name: 'New Exercise',
        description: 'Exercise description',
        sets: [{ reps: null, weight: null, duration: null }],
        restTime: null,
      })
    "
  />
</template>

<script lang="ts" setup>
import type { Exercise } from "~/types/Exercise/Exercise";
import { v4 as uuid } from "uuid";

interface Props {
  modelValue: Exercise[] | undefined;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
});

const emit = defineEmits(["update:modelValue"]);

function addExercise(exercise: Exercise) {
  const newExercises = [...props.modelValue, exercise];
  emit("update:modelValue", newExercises);
}

function removeExercise(index: number) {
  console.debug("Removing exercise at index:", index);
  const newExercises = props.modelValue.filter((_, i) => i !== index);
  emit("update:modelValue", newExercises);
}
</script>
