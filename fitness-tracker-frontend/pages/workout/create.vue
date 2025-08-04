<template>
  <PageTitle class="mb-4">{{ $t("workout.create.title") }}</PageTitle>
  <UForm :schema="schema" :state="state" @submit.prevent="onSubmit" class="space-y-4">
    <UFormField :label="$t('workout.create.form.name')" name="name">
      <UInput v-model="state.name" />
    </UFormField>
    <UFormField :label="$t('workout.create.form.exercises')" name="exercises">
      <ExerciseSelector v-model="state.exercises" />
    </UFormField>
    <UButton type="submit" :label="$t('buttons.submit')" @click="console.log('submit')" />
  </UForm>
</template>

<script lang="ts" setup>
import * as zod from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";
import { ExerciseSchema } from "~/types/Exercise/Exercise";
import { WorkoutCreateSchema, type WorkoutCreate } from "~/types/workout/workoutCreate";

const toast = useToast();

const schema = zod.object({
  name: zod.string().min(1, "Name is required"),
  exercises: zod.array(ExerciseSchema).nonempty("At least one exercise is required"),
});
type Schema = zod.infer<typeof schema>;
const state = reactive<Partial<Schema>>({
  name: "",
  exercises: [],
});

async function onSubmit(event: FormSubmitEvent<Schema>) {
  if (event.data) {
    const createData: WorkoutCreate = {
      name: event.data.name,
      exercises: event.data.exercises.map((exercise) => ({
        uuid: exercise.uuid,
        sets: exercise.sets.map((set) => ({
          reps: set.reps,
          weight: set.weight,
          duration: set.duration,
        })),
      })),
    };
    const data = WorkoutCreateSchema.safeParse(createData);
    if (data.success && data.data) {
      // TODO Create API request
      console.debug("Submitting workout create data:", data.data);
    } else {
      toast.add({
        title: "Unable to create workout",
        description: "Something went wrong while creating the workout",
        icon: "i-lucide-alert-triangle",
        color: "error",
      });
    }
  } else {
    console.error("Form validation failed");
  }
}

useHead({
  title: $t("workout.create.title"),
  meta: [
    {
      name: "description",
      content: $t("workout.create.description"),
    },
  ],
});
</script>
