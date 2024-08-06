import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { hashPassword } from "../../../services";

const signup: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.route({
    method: "POST",
    url: "/",
    schema: {
      tags: ["Auth"],
      body: z.object({
        username: z.string().min(3).max(25),
        password: z
          .string()
          .min(6, { message: "min password is of length 6" })
          .max(25),
      }),
      response: {
        201: z.object({
          id: z.string(),
          username: z.string(),
        }),
        400: z.object({
          message: z.string(),
        }),
      },
    },
    handler: async (request, reply) => {
      const { username, password } = request.body;
      try {
        const hash = await hashPassword(password, fastify);
        const user = await fastify.prisma.user.create({
          data: {
            username,
            password: hash,
          },
        });
        const data = {
          id: user.id,
          username: user.username,
        };
        return reply.status(201).send(data);
      } catch (error) {
        return reply.status(400).send({ message: "cannot create user" });
      }
    },
  });
};

export default signup;
