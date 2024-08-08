"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const root = async (fastify, _opts) => {
    fastify.route({
        method: "GET",
        url: "/ping",
        schema: {
            tags: ["Healthcheck"],
            response: {
                200: zod_1.z.object({
                    message: zod_1.z.string(),
                }),
            },
        },
        handler: async () => {
            return { message: "PONG!" };
        },
    });
};
exports.default = root;
