"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.signUserJwt = exports.verifyUserJwt = exports.verifyPassword = exports.hashPassword = void 0;
const fastify_plugin_1 = __importDefault(require("fastify-plugin"));
const hashPassword = async (password, fastify) => {
    const hash = await fastify.bcrypt.hash(password);
    return hash;
};
exports.hashPassword = hashPassword;
const verifyPassword = async (password, hash, fastify) => {
    const verify = await fastify.bcrypt.compare(password, hash);
    return verify;
};
exports.verifyPassword = verifyPassword;
exports.verifyUserJwt = (0, fastify_plugin_1.default)(async (fastify) => {
    fastify.decorate("verifyJwt", (token) => {
        const verify = fastify.jwt.verify(token);
        return verify;
    });
}, { name: "verifyJwt" });
exports.signUserJwt = (0, fastify_plugin_1.default)(async (fastify) => {
    fastify.decorate("signUser", (user) => {
        const signed = fastify.jwt.sign(user);
        return signed;
    });
}, { name: "signUser" });
