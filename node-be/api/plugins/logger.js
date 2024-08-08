"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fastify_plugin_1 = __importDefault(require("fastify-plugin"));
const fastify_request_logger_1 = __importDefault(require("@mgcrea/fastify-request-logger"));
exports.default = (0, fastify_plugin_1.default)(async (fastify) => {
    await fastify.register(fastify_request_logger_1.default);
});
