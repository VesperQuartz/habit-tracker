"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fastify_plugin_1 = __importDefault(require("fastify-plugin"));
const client_1 = require("@prisma/client");
const fastify_prisma_1 = __importDefault(require("@joggr/fastify-prisma"));
exports.default = (0, fastify_plugin_1.default)(async (fastify) => {
    await fastify.register(fastify_prisma_1.default, {
        client: new client_1.PrismaClient(),
        clientConfig: {
            log: [{ emit: "event", level: "query" }],
        },
    });
});
