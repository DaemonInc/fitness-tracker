<template>
  <PageTitle class="mb-4">{{ $t("workout.create.title") }}</PageTitle>
  <UForm :schema="schema" :state="state" @submit.prevent="onSubmit" class="space-y-4">
    <UFormField :label="$t('workout.create.form.name')" name="name">
      <UInput v-model="state.name" />
    </UFormField>
    <UFormField :label="$t('workout.create.form.exercises')" name="exercises">
      <ExerciseSelector v-model="state.exercises" />
    </UFormField>
    <UButton
      type="submit"
      loading-auto
      :label="$t('buttons.submit')"
      @click="console.log('submit')"
    />
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
      await new Promise((resolve) => setTimeout(resolve, 2000));
      toast.add({
        title: $t('workout.form.workout_created_title'),
        description: $t('workout.form.workout_created_description'),
        icon: "i-lucide-check-circle",
        color: "success",
      });
      useRouter().replace("/workout");
      return;
    }
  }

  toast.add({
    title: $t("errors.create_workout_title"),
    description: $t("errors.create_workout_description"),
    icon: "i-lucide-alert-triangle",
    color: "error",
  });
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
