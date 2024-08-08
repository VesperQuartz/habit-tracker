"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const schema_1 = require("./schema");
const habit = async (fastify, _opts) => {
    fastify.route({
        method: "POST",
        url: "/",
        schema: {
            tags: ["Habit"],
            body: schema_1.HabitReqSchema,
            response: {
                200: schema_1.HabitResSchema,
            },
        },
        handler: async (req) => {
            const { name, description, startDate, frequency, userId } = req.body;
            try {
                const habit = await fastify.prisma.habit.create({
                    data: {
                        frequency,
                        name,
                        startDate,
                        description,
                        userId,
                    },
                });
                if (habit) {
                    return { ...habit, message: "Habit created successfully!" };
                }
                else {
                    throw new Error("Failed to create habit");
                }
            }
            catch (error) {
                console.error(error);
            }
        },
    });
    fastify.route({
        method: "GET",
        url: "/:id",
        schema: {
            tags: ["Habit"],
            params: zod_1.z.object({
                id: zod_1.z.string(),
            }),
            response: {
                200: schema_1.UserHabitResponse,
            },
        },
        handler: async (req) => {
            const { id } = req.params;
            try {
                const userHabit = await fastify.prisma.habit.findUnique({
                    where: {
                        id,
                    },
                    include: {
                        logs: true,
                        reminders: true,
                    },
                });
                if (userHabit) {
                    return userHabit;
                }
                throw new Error("Habit not found");
            }
            catch (error) {
                console.error(error);
            }
        },
    });
    fastify.route({
        method: "PATCH",
        url: "/:id",
        schema: {
            tags: ["Habit"],
            params: zod_1.z.object({
                id: zod_1.z.string(),
            }),
            body: zod_1.z.object({
                name: zod_1.z.string(),
            }),
            response: {
                200: zod_1.z.object({
                    message: zod_1.z.string(),
                }),
            },
        },
        handler: async (req) => {
            const { id } = req.params;
            const { name } = req.body;
            try {
                await fastify.prisma.habit.update({
                    where: {
                        id,
                    },
                    data: {
                        name,
                    },
                });
                return { message: "name has been updated!" };
            }
            catch (error) {
                console.error(error);
            }
        },
    });
    fastify.route({
        method: "DELETE",
        url: "/:id",
        schema: {
            tags: ["Habit"],
            params: zod_1.z.object({
                id: zod_1.z.string(),
            }),
            response: {
                200: zod_1.z.object({
                    message: zod_1.z.string(),
                }),
            },
        },
        handler: async (req) => {
            const { id } = req.params;
            try {
                await fastify.prisma.habit.delete({
                    where: {
                        id,
                    },
                });
                return { message: "Task deleted sucessfully!" };
            }
            catch (error) {
                console.error(error);
            }
        },
    });
};
exports.default = habit;
