import { z } from "zod";
import { ExerciseSetSchema } from "../Exercise/ExerciseSet";

const WorkoutExerciseSchema = z.object({
  uuid: z.uuid(),
  sets: z.array(ExerciseSetSchema).nonempty(),
});

export const WorkoutCreateSchema = z.object({
  name: z.string().min(1),
  exercises: z.array(WorkoutExerciseSchema).nonempty(),
});

export type WorkoutCreate = z.infer<typeof WorkoutCreateSchema>;