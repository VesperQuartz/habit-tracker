import { FastifyInstance, FastifyPluginCallback } from "fastify";
import fp from "fastify-plugin";

export const hashPassword = async (
  password: string,
  fastify: FastifyInstance,
) => {
  const hash = await fastify.bcrypt.hash(password);
  return hash;
};

export const verifyPassword = async (
  password: string,
  hash: string,
  fastify: FastifyInstance,
) => {
  const verify = await fastify.bcrypt.compare(password, hash);
  return verify;
};

export const verifyUserJwt = fp(
  async (fastify) => {
    fastify.decorate("verifyJwt", (token: string) => {
      const verify = fastify.jwt.verify(token);
      return verify;
    });
  },
  { name: "verifyJwt" },
);

export const signUserJwt: FastifyPluginCallback = fp(
  async (fastify) => {
    fastify.decorate("signUser", <T extends object>(user: T) => {
      const signed = fastify.jwt.sign(user);
      return signed;
    });
  },
  { name: "signUser" },
);
