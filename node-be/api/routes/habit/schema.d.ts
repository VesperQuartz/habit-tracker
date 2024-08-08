import z from "zod";
export declare const HabitResSchema: z.ZodObject<{
    id: z.ZodString;
    userId: z.ZodString;
    name: z.ZodString;
    description: z.ZodOptional<z.ZodNullable<z.ZodString>>;
    startDate: z.ZodDate;
    frequency: z.ZodString;
    createdAt: z.ZodDate;
    updatedAt: z.ZodDate;
    message: z.ZodOptional<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    name: string;
    id: string;
    createdAt: Date;
    updatedAt: Date;
    userId: string;
    startDate: Date;
    frequency: string;
    description?: string | null | undefined;
    message?: string | undefined;
}, {
    name: string;
    id: string;
    createdAt: Date;
    updatedAt: Date;
    userId: string;
    startDate: Date;
    frequency: string;
    description?: string | null | undefined;
    message?: string | undefined;
}>;
export declare const HabitReqSchema: z.ZodObject<{
    name: z.ZodString;
    description: z.ZodOptional<z.ZodString>;
    startDate: z.ZodDate;
    frequency: z.ZodEnum<["Daily", "Weekly"]>;
    userId: z.ZodString;
}, "strip", z.ZodTypeAny, {
    name: string;
    userId: string;
    startDate: Date;
    frequency: "Daily" | "Weekly";
    description?: string | undefined;
}, {
    name: string;
    userId: string;
    startDate: Date;
    frequency: "Daily" | "Weekly";
    description?: string | undefined;
}>;
export declare const HabitLogSchema: z.ZodObject<{
    id: z.ZodString;
    habitId: z.ZodString;
    date: z.ZodDate;
    status: z.ZodEnum<["Done", "Missed", "Pending"]>;
    createdAt: z.ZodDate;
}, "strip", z.ZodTypeAny, {
    date: Date;
    status: "Done" | "Missed" | "Pending";
    id: string;
    createdAt: Date;
    habitId: string;
}, {
    date: Date;
    status: "Done" | "Missed" | "Pending";
    id: string;
    createdAt: Date;
    habitId: string;
}>;
export declare const ReminderSchema: z.ZodObject<{
    id: z.ZodString;
    habitId: z.ZodString;
    reminderTime: z.ZodDate;
    method: z.ZodEnum<["Email", "SMS", "PushNotification"]>;
    createdAt: z.ZodDate;
    updatedAt: z.ZodDate;
}, "strip", z.ZodTypeAny, {
    method: "Email" | "SMS" | "PushNotification";
    id: string;
    createdAt: Date;
    updatedAt: Date;
    habitId: string;
    reminderTime: Date;
}, {
    method: "Email" | "SMS" | "PushNotification";
    id: string;
    createdAt: Date;
    updatedAt: Date;
    habitId: string;
    reminderTime: Date;
}>;
export declare const UserHabitResponse: z.ZodObject<z.objectUtil.extendShape<{
    logs: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        habitId: z.ZodString;
        date: z.ZodDate;
        status: z.ZodEnum<["Done", "Missed", "Pending"]>;
        createdAt: z.ZodDate;
    }, "strip", z.ZodTypeAny, {
        date: Date;
        status: "Done" | "Missed" | "Pending";
        id: string;
        createdAt: Date;
        habitId: string;
    }, {
        date: Date;
        status: "Done" | "Missed" | "Pending";
        id: string;
        createdAt: Date;
        habitId: string;
    }>, "many">>;
    reminder: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        habitId: z.ZodString;
        reminderTime: z.ZodDate;
        method: z.ZodEnum<["Email", "SMS", "PushNotification"]>;
        createdAt: z.ZodDate;
        updatedAt: z.ZodDate;
    }, "strip", z.ZodTypeAny, {
        method: "Email" | "SMS" | "PushNotification";
        id: string;
        createdAt: Date;
        updatedAt: Date;
        habitId: string;
        reminderTime: Date;
    }, {
        method: "Email" | "SMS" | "PushNotification";
        id: string;
        createdAt: Date;
        updatedAt: Date;
        habitId: string;
        reminderTime: Date;
    }>, "many">>;
}, {
    id: z.ZodString;
    userId: z.ZodString;
    name: z.ZodString;
    description: z.ZodOptional<z.ZodNullable<z.ZodString>>;
    startDate: z.ZodDate;
    frequency: z.ZodString;
    createdAt: z.ZodDate;
    updatedAt: z.ZodDate;
    message: z.ZodOptional<z.ZodString>;
}>, "strip", z.ZodTypeAny, {
    name: string;
    id: string;
    createdAt: Date;
    updatedAt: Date;
    userId: string;
    startDate: Date;
    frequency: string;
    description?: string | null | undefined;
    message?: string | undefined;
    logs?: {
        date: Date;
        status: "Done" | "Missed" | "Pending";
        id: string;
        createdAt: Date;
        habitId: string;
    }[] | undefined;
    reminder?: {
        method: "Email" | "SMS" | "PushNotification";
        id: string;
        createdAt: Date;
        updatedAt: Date;
        habitId: string;
        reminderTime: Date;
    }[] | undefined;
}, {
    name: string;
    id: string;
    createdAt: Date;
    updatedAt: Date;
    userId: string;
    startDate: Date;
    frequency: string;
    description?: string | null | undefined;
    message?: string | undefined;
    logs?: {
        date: Date;
        status: "Done" | "Missed" | "Pending";
        id: string;
        createdAt: Date;
        habitId: string;
    }[] | undefined;
    reminder?: {
        method: "Email" | "SMS" | "PushNotification";
        id: string;
        createdAt: Date;
        updatedAt: Date;
        habitId: string;
        reminderTime: Date;
    }[] | undefined;
}>;
