<template>
  <template v-for="exercise in modelValue" :key="exercise.uuid">
    <ExerciseEditor class="mb-4" :model-value="exercise" />
  </template>
  <UButton
    label="Add Exercise"
    @click="
      addExercise({
        uuid: '',
        name: 'New Exercise',
        description: 'Exercise description',
        sets: [
          { reps: null, weight: null, duration: null },
        ],
        restTime: null,
      })
    "
  />
</template>

<script lang="ts" setup>
import type { Exercise } from "~/types/Exercise";

interface Props {
  modelValue: Exercise[] | undefined;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
});

const emit = defineEmits<(e: "update:modelValue", value: Exercise[]) => void>();

function addExercise(exercise: Exercise) {
  const newExercises = [...props.modelValue, exercise];
  emit("update:modelValue", newExercises);
}

function removeExercise(index: number) {
  const newExercises = props.modelValue.filter((_, i) => i !== index);
  emit("update:modelValue", newExercises);
}

function updateExercise(index: number, uuid: Exercise) {
  const newExercises = [...props.modelValue];
  newExercises[index] = uuid;
  emit("update:modelValue", newExercises);
}
</script>
