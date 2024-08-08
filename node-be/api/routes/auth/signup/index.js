"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const services_1 = require("../../../services");
const signup = async (fastify, _opts) => {
    fastify.route({
        method: "POST",
        url: "/",
        schema: {
            tags: ["Auth"],
            body: zod_1.z.object({
                username: zod_1.z.string().min(3).max(25),
                password: zod_1.z
                    .string()
                    .min(6, { message: "min password is of length 6" })
                    .max(25),
            }),
            response: {
                201: zod_1.z.object({
                    id: zod_1.z.string(),
                    username: zod_1.z.string(),
                }),
                400: zod_1.z.object({
                    message: zod_1.z.string(),
                }),
            },
        },
        handler: async (request, reply) => {
            const { username, password } = request.body;
            try {
                const hash = await (0, services_1.hashPassword)(password, fastify);
                const user = await fastify.prisma.user.create({
                    data: {
                        username,
                        password: hash,
                    },
                });
                const data = {
                    id: user.id,
                    username: user.username,
                };
                return reply.status(201).send(data);
            }
            catch (error) {
                return reply.status(400).send({ message: "cannot create user" });
            }
        },
    });
};
exports.default = signup;
