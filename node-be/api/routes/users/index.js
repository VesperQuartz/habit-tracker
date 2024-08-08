"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const schema_1 = require("./schema");
const users = async (fastify, _opts) => {
    fastify.route({
        method: "GET",
        url: "/:id",
        schema: {
            tags: ["User"],
            params: zod_1.z.object({
                id: zod_1.z.string(),
            }),
            response: {
                200: schema_1.UserWithoutPassword,
            },
        },
        handler: async (req) => {
            const { id } = req.params;
            try {
                const users = await fastify.prisma.user.findUnique({
                    where: {
                        id,
                    },
                    include: {
                        habits: {
                            include: {
                                reminders: true,
                                logs: true,
                            },
                        },
                    },
                });
                if (users) {
                    const { password, ...rest } = users;
                    fastify.log.info(rest);
                    return rest;
                }
                else {
                    throw new Error("User not found");
                }
            }
            catch (error) {
                console.error(error);
            }
        },
    });
};
exports.default = users;
