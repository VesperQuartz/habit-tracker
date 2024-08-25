import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { HabitResSchema } from "../../../habit/schema";

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
          habits: z.array(HabitResSchema),
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
                  equals: "Daily",
                },
              },
              orderBy: {
                createdAt: "desc",
              },
            },
          },
        });
        if (habit) {
          return habit;
        } else {
          throw new Error("User does not have any habits");
        }
      } catch (error) {
        console.error(error);
      }
    },
  });
  fastify.route({
    method: "GET",
    url: "/:id/count",
    schema: {
      tags: ["User"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: z.object({
          count: z.number(),
        }),
      },
    },
    handler: async (req) => {
      const { id } = req.params;
      try {
        const habit = await fastify.prisma.habit.count({
          where: {
            userId: id,
            frequency: "Daily",
          },
        });
        if (habit) {
          return { count: habit };
        } else {
          throw new Error("User does not have any habits");
        }
      } catch (error) {
        console.error(error);
      }
    },
  });
};

export default userHabit;
