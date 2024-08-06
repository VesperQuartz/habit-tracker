import z from "zod";

export const HabitResSchema = z.object({
  id: z.string(),
  userId: z.string(),
  name: z.string(),
  description: z.string().nullish(),
  startDate: z.date(),
  frequency: z.string(),
  createdAt: z.date(),
  updatedAt: z.date(),
  message: z.string().optional(),
});

export const HabitReqSchema = z.object({
  name: z.string(),
  description: z.string().optional(),
  startDate: z.coerce.date(),
  frequency: z.enum(["Daily", "Weekly"]),
  userId: z.string(),
});

export const HabitLogSchema = z.object({
  id: z.string(),
  habitId: z.string(),
  date: z.date(),
  status: z.enum(["Done", "Missed", "Pending"]),
  createdAt: z.date(),
});

export const ReminderSchema = z.object({
  id: z.string(),
  habitId: z.string(),
  reminderTime: z.date(),
  method: z.enum(["Email", "SMS", "PushNotification"]),
  createdAt: z.date(),
  updatedAt: z.date(),
});

export const UserHabitResponse = z
  .object({
    logs: z.array(HabitLogSchema).optional(),
    reminder: z.array(ReminderSchema).optional(),
  })
  .merge(HabitResSchema);
