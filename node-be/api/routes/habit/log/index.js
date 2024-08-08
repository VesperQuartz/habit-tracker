"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const log = async (fastify, _opts) => {
    fastify.route({
        method: "POST",
        url: "/id",
        schema: {
            tags: ["Logs"],
            body: zod_1.z.object({
                habitId: zod_1.z.string(),
                date: zod_1.z.coerce.date(),
                status: zod_1.z.enum(["Done", "Missed", "Pending"]),
            }),
            response: {
                200: zod_1.z.object({
                    message: zod_1.z.string(),
                }),
            },
        },
        handler: async (req) => {
            const { habitId, date, status } = req.body;
            try {
                fastify.prisma.habitLog.create({
                    data: {
                        habitId,
                        date,
                        status,
                    },
                });
                return { message: "Log created successfully!" };
            }
            catch (error) {
                console.error(error);
            }
        },
    });
};
exports.default = log;
