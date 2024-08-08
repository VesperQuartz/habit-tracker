"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserWithoutPassword = void 0;
const zod_1 = __importDefault(require("zod"));
const schema_1 = require("../habit/schema");
exports.UserWithoutPassword = zod_1.default.object({
    id: zod_1.default.string(),
    username: zod_1.default.string(),
    createdAt: zod_1.default.date(),
    updatedAt: zod_1.default.date(),
    habits: zod_1.default.array(schema_1.UserHabitResponse),
});
