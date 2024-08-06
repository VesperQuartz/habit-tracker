import z from "zod";
import { UserHabitResponse } from "../habit/schema";

export const UserWithoutPassword = z.object({
  id: z.string(),
  username: z.string(),
  createdAt: z.date(),
  updatedAt: z.date(),
  habits: z.array(UserHabitResponse),
});
