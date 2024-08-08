"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.options = exports.app = void 0;
const path_1 = require("path");
const autoload_1 = __importDefault(require("@fastify/autoload"));
const swagger_1 = __importDefault(require("@fastify/swagger"));
const swagger_ui_1 = __importDefault(require("@fastify/swagger-ui"));
const fastify_type_provider_zod_1 = require("fastify-type-provider-zod");
import("@mgcrea/pino-pretty-compact");
const options = {
    logger: {
        level: "debug",
        transport: {
            target: "@mgcrea/pino-pretty-compact",
            options: {
                translateTime: "HH:MM:ss Z",
                ignore: "pid,hostname",
            },
        },
    },
};
exports.options = options;
const app = async (fastify, opts) => {
    fastify.setValidatorCompiler(fastify_type_provider_zod_1.validatorCompiler);
    fastify.setSerializerCompiler(fastify_type_provider_zod_1.serializerCompiler);
    fastify.register(swagger_1.default, {
        openapi: {
            info: {
                title: "Habit Tracker",
                description: "The Habit Tracker is a simple tool that helps you build and maintain positive habits by tracking your progress.",
                version: "1.0.0",
            },
            servers: [],
        },
        transform: fastify_type_provider_zod_1.jsonSchemaTransform,
    });
    fastify.register(swagger_ui_1.default, {
        routePrefix: "/docs",
    });
    void fastify.register(autoload_1.default, {
        dir: (0, path_1.join)(__dirname, "plugins"),
        options: opts,
    });
    void fastify.register(autoload_1.default, {
        dir: (0, path_1.join)(__dirname, "routes"),
        options: opts,
    });
};
exports.app = app;
exports.default = app;
