import { FastifyInstance, FastifyPluginCallback } from "fastify";
export declare const hashPassword: (password: string, fastify: FastifyInstance) => Promise<string>;
export declare const verifyPassword: (password: string, hash: string, fastify: FastifyInstance) => Promise<boolean>;
export declare const verifyUserJwt: (fastify: FastifyInstance<import("fastify").RawServerDefault, import("http").IncomingMessage, import("http").ServerResponse<import("http").IncomingMessage>, import("fastify").FastifyBaseLogger, import("fastify").FastifyTypeProviderDefault>) => Promise<void>;
export declare const signUserJwt: FastifyPluginCallback;
