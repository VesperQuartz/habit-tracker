"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fastify_plugin_1 = __importDefault(require("fastify-plugin"));
const fastify_bcrypt_1 = __importDefault(require("fastify-bcrypt"));
exports.default = (0, fastify_plugin_1.default)(async (fastify) => {
    await fastify.register(fastify_bcrypt_1.default, {
        saltWorkFactor: 12,
    });
});
