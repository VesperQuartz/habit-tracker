import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";

const log: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.route({
    method: "POST",
    url: "/",
    schema: {
      tags: ["Logs"],
      body: z.object({
        habitId: z.string(),
        date: z.coerce.date(),
        status: z.enum(["Done", "Missed", "Pending"]),
      }),
      response: {
        200: z.object({
          message: z.string(),
        }),
      },
    },
    handler: async (req) => {
      const { habitId, date, status } = req.body;
      try {
        fastify.prisma.habitLog.create({
          data: {
            habitId,
            date,
            status,
          },
        });
        return { message: "Log created successfully!" };
      } catch (error) {
        console.error(error);
      }
    },
  });
};

export default log;
