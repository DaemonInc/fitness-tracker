<template>
  <PageTitle class="mb-4">{{ $t("workout.create.title") }}</PageTitle>
  <UForm :schema="schema" :state="state" @submit="onSubmit" class="space-y-4">
    <UFormField :label="$t('workout.create.form.name')" name="name">
      <UInput v-model="state.name" />
    </UFormField>
    <UFormField :label="$t('workout.create.form.exercises')" name="exercises">
      <ExerciseSelector v-model="state.exercises" />
    </UFormField>
    <UButton type="submit" :label="$t('buttons.submit')" />
  </UForm>
</template>

<script lang="ts" setup>
import * as zod from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";
import { ExerciseSchema } from "~/types/Exercise";

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
  event.preventDefault();
  if (event.data) {
    console.log("Form submitted with data:", event.data);
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
