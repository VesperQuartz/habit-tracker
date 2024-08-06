import fp from "fastify-plugin";
import { signUserJwt, verifyUserJwt } from "../services";

export default fp(async (fastify) => {
  await fastify.register(verifyUserJwt);
  await fastify.register(signUserJwt);
});
