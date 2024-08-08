"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserHabitResponse = exports.ReminderSchema = exports.HabitLogSchema = exports.HabitReqSchema = exports.HabitResSchema = void 0;
const zod_1 = __importDefault(require("zod"));
exports.HabitResSchema = zod_1.default.object({
    id: zod_1.default.string(),
    userId: zod_1.default.string(),
    name: zod_1.default.string(),
    description: zod_1.default.string().nullish(),
    startDate: zod_1.default.date(),
    frequency: zod_1.default.string(),
    createdAt: zod_1.default.date(),
    updatedAt: zod_1.default.date(),
    message: zod_1.default.string().optional(),
});
exports.HabitReqSchema = zod_1.default.object({
    name: zod_1.default.string(),
    description: zod_1.default.string().optional(),
    startDate: zod_1.default.coerce.date(),
    frequency: zod_1.default.enum(["Daily", "Weekly"]),
    userId: zod_1.default.string(),
});
exports.HabitLogSchema = zod_1.default.object({
    id: zod_1.default.string(),
    habitId: zod_1.default.string(),
    date: zod_1.default.date(),
    status: zod_1.default.enum(["Done", "Missed", "Pending"]),
    createdAt: zod_1.default.date(),
});
exports.ReminderSchema = zod_1.default.object({
    id: zod_1.default.string(),
    habitId: zod_1.default.string(),
    reminderTime: zod_1.default.date(),
    method: zod_1.default.enum(["Email", "SMS", "PushNotification"]),
    createdAt: zod_1.default.date(),
    updatedAt: zod_1.default.date(),
});
exports.UserHabitResponse = zod_1.default
    .object({
    logs: zod_1.default.array(exports.HabitLogSchema).optional(),
    reminder: zod_1.default.array(exports.ReminderSchema).optional(),
})
    .merge(exports.HabitResSchema);
