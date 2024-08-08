"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const services_1 = require("../../../services");
const login = async (fastify, _opts) => {
    fastify.decorate("user", { username: "", password: "" });
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
                200: zod_1.z.object({
                    message: zod_1.z.string(),
                    user: zod_1.z.object({
                        id: zod_1.z.string(),
                        username: zod_1.z.string(),
                        token: zod_1.z.string(),
                    }),
                }),
            },
        },
        handler: async (request) => {
            const { username, password } = request.body;
            try {
                const user = await fastify.prisma.user.findUnique({
                    where: { username },
                });
                if (!user) {
                    throw new Error("User not found");
                }
                const passwordMatch = await (0, services_1.verifyPassword)(password, user.password, fastify);
                if (!passwordMatch) {
                    throw new Error("username or password incorrect");
                }
                const token = fastify.jwt.sign({
                    id: user.id,
                    username: user.username,
                });
                return {
                    message: "sign in was a success",
                    user: {
                        id: user.id,
                        username: user.username,
                        token,
                    },
                };
            }
            catch (error) {
                if (error instanceof Error) {
                    throw new Error(error.message);
                }
            }
        },
    });
};
exports.default = login;
