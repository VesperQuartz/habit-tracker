import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { HabitReqSchema, HabitResSchema, UserHabitResponse } from "./schema";

const habit: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.route({
    method: "POST",
    url: "/",
    schema: {
      tags: ["Habit"],
      body: HabitReqSchema,
      response: {
        200: HabitResSchema,
      },
    },
    handler: async (req) => {
      const { name, description, startDate, frequency, userId } = req.body;
      try {
        const habit = await fastify.prisma.habit.create({
          data: {
            frequency,
            name,
            startDate,
            description,
            userId,
          },
        });
        if (habit) {
          return { ...habit, message: "Habit created successfully!" };
        } else {
          throw new Error("Failed to create habit");
        }
      } catch (error) {
        console.error(error);
      }
    },
  });
  fastify.route({
    method: "GET",
    url: "/:id",
    schema: {
      tags: ["Habit"],
      params: z.object({
        id: z.string(),
      }),
      response: {
        200: UserHabitResponse,
      },
    },
    handler: async (req) => {
      const { id } = req.params;
      try {
        const userHabit = await fastify.prisma.habit.findUnique({
          where: {
            id,
          },
          include: {
            logs: true,
            reminders: true,
          },
        });
        if (userHabit) {
          return userHabit;
        }
        throw new Error("Habit not found");
      } catch (error) {
        console.error(error);
      }
    },
  });

  fastify.route({
    method: "PATCH",
    url: "/:id",
    schema: {
      tags: ["Habit"],
      params: z.object({
        id: z.string(),
      }),
      body: z.object({
        name: z.string(),
      }),
      response: {
        200: z.object({
          message: z.string(),
        }),
      },
    },
    handler: async (req) => {
      const { id } = req.params;
      const { name } = req.body;
      try {
        await fastify.prisma.habit.update({
          where: {
            id,
          },
          data: {
            name,
          },
        });
        return { message: "name has been updated!" };
      } catch (error) {
        console.error(error);
      }
    },
  });
  fastify.route({
    method: "DELETE",
    url: "/:id",
    schema: {
      tags: ["Habit"],
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
        await fastify.prisma.habit.delete({
          where: {
            id,
          },
        });
        return { message: "Habit deleted sucessfully!" };
      } catch (error) {
        fastify.log.error(error);
        throw new Error("Failed to delete habit");
      }
    },
  });
};

export default habit;
