import fp from "fastify-plugin";
import bcrypt from "fastify-bcrypt";

export default fp(async (fastify) => {
  await fastify.register(bcrypt, {
    saltWorkFactor: 12,
  });
});
