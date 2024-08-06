import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { UserWithoutPassword } from "./schema";

const users: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.route({
    method: "GET",
    url: "/:id",
    schema: {
      tags: ["User"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: UserWithoutPassword,
      },
    },
    handler: async (req) => {
      const { id } = req.params;
      try {
        const users = await fastify.prisma.user.findUnique({
          where: {
            id,
          },
          include: {
            habits: {
              include: {
                reminders: true,
                logs: true,
              },
            },
          },
        });
        if (users) {
          const { password, ...rest } = users;
          fastify.log.info(rest);
          return rest;
        } else {
          throw new Error("User not found");
        }
      } catch (error) {
        console.error(error);
      }
    },
  });
};

export default users;
