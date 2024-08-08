"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fastify_plugin_1 = __importDefault(require("fastify-plugin"));
const services_1 = require("../services");
exports.default = (0, fastify_plugin_1.default)(async (fastify) => {
    await fastify.register(services_1.verifyUserJwt);
    await fastify.register(services_1.signUserJwt);
});
