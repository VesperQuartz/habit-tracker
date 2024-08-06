import fp from "fastify-plugin";
import fastifyRequestLogger from "@mgcrea/fastify-request-logger";

export default fp(async (fastify) => {
  await fastify.register(fastifyRequestLogger);
});
