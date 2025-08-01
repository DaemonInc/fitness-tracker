import { z } from "zod";
import { ExerciseSetSchema } from "./ExerciseSet";

export const ExerciseSchema = z.object({
  uuid: z.string(),
  name: z.string(),
  description: z.string(),
  sets: z.array(ExerciseSetSchema),
  restTime: z.number().nullable(),
});

export type Exercise = z.infer<typeof ExerciseSchema>;
