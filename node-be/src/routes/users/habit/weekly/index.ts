import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";

const userHabit: FastifyPluginAsyncZod = async (
  fastify,
  _opts,
): Promise<void> => {
  fastify.route({
    method: "GET",
    url: "/:id",
    schema: {
      tags: ["User"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: z.object({
          message: z.string(),
        }),
      },
    },
    handler: async (req) => {
      const { id } = req.params;
      try {
        const habit = await fastify.prisma.user.findUnique({
          where: {
            id,
          },
          select: {
            habits: {
              where: {
                frequency: {
                  equals: "Weekly",
                },
              },
            },
            _count: true,
          },
        });
        fastify.log.info(`habit: ${JSON.stringify(habit)}`);
      } catch (error) {
        console.error(error);
      }
      return { message: "PONG!" };
    },
  });
};

export default userHabit;
