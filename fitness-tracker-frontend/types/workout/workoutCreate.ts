import { z } from "zod";
import { ExerciseSetSchema } from "../Exercise/ExerciseSet";

const WorkoutExerciseSchema = z.object({
  uuid: z.uuid(),
  sets: z.array(ExerciseSetSchema).nonempty(),
});

export const WorkoutCreateSchema = z.object({
  name: z.string().min(1, "Name is required"),
  exercises: z.array(WorkoutExerciseSchema).nonempty("At least one exercise is required"),
});

export type WorkoutCreate = z.infer<typeof WorkoutCreateSchema>;