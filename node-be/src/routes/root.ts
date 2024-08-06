import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";

const root: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.route({
    method: "GET",
    url: "/ping",
    schema: {
      tags: ["Healthcheck"],
      response: {
        200: z.object({
          message: z.string(),
        }),
      },
    },
    handler: async () => {
      return { message: "PONG!" };
    },
  });
};

export default root;
