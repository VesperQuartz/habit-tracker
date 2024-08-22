import { z } from "zod";
import { startOfDay, endOfDay, startOfWeek } from "date-fns";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { HabitLogWDSchema } from "../schema";

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
        const existingLog = await fastify.prisma.habitLog.findFirst({
          where: {
            habitId: habitId,
            date: date,
          },
        });
        fastify.log.info(`is existing: ${existingLog}`);
        if (!existingLog) {
          await fastify.prisma.habitLog.create({
            data: {
              habitId,
              date,
              status,
            },
          });
          return { message: "Log created successfully!!!" };
        } else {
          throw new Error("Log already exists!!");
        }
      } catch (error) {
        if (error instanceof Error) {
          throw new Error(error.message);
        }
        throw new Error("something went wrong");
      }
    },
  });
  fastify.route({
    method: "GET",
    url: "/:id/daily",
    schema: {
      tags: ["Logs"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: z.array(HabitLogWDSchema),
      },
    },
    handler: async (req) => {
      const today = new Date();
      const startOfToday = startOfDay(today);
      const endOfToday = endOfDay(today);
      const { id } = req.params;
      try {
        const dailylogs = await fastify.prisma.habitLog.findMany({
          where: {
            date: {
              gte: startOfToday,
              lte: endOfToday,
            },
            status: "Done",
            habit: {
              userId: id,
              frequency: "Daily",
            },
          },
          include: {
            habit: true,
          },
        });
        if (dailylogs) {
          fastify.log.info(`daily logs: ${JSON.stringify(dailylogs)}`);
          return dailylogs;
        } else {
          throw new Error("No daily logs found");
        }
      } catch (error) {
        if (error instanceof Error) {
          throw new Error(error.message);
        }
        throw new Error("something went wrong");
      }
    },
  });
  fastify.route({
    method: "GET",
    url: "/:id/weekly",
    schema: {
      tags: ["Logs"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: z.array(HabitLogWDSchema),
      },
    },
    handler: async (req) => {
      const today = new Date();
      const endOfDay = new Date(today.setHours(23, 59, 59, 999));
      const startOfThisWeek = startOfWeek(today, { weekStartsOn: 1 });
      const { id } = req.params;
      try {
        const weeklyLogs = await fastify.prisma.habitLog.findMany({
          where: {
            date: {
              gte: startOfThisWeek,
              lte: endOfDay,
            },
            status: "Done",
            habit: {
              userId: id,
              frequency: "Weekly",
            },
          },
          include: {
            habit: true,
          },
        });
        if (weeklyLogs) {
          fastify.log.info(`daily logs: ${JSON.stringify(weeklyLogs)}`);
          return weeklyLogs;
        } else {
          throw new Error("No weekly logs found");
        }
      } catch (error) {
        if (error instanceof Error) {
          throw new Error(error.message);
        }
        throw new Error("something went wrong");
      }
    },
  });
};

export default log;
