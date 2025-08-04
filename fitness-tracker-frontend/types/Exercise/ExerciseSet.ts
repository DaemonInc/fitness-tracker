import { z } from "zod";

export const ExerciseSetSchema = z.object({
  reps: z.number().nullable(),
  weight: z.number().nullable(),
  duration: z.number().nullable(),
});

export type ExerciseSet = z.infer<typeof ExerciseSetSchema>;