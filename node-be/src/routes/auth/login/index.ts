import { z } from "zod";
import { FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { verifyPassword } from "../../../services";
const login: FastifyPluginAsyncZod = async (fastify, _opts): Promise<void> => {
  fastify.decorate("user", { username: "", password: "" });
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
        200: z.object({
          message: z.string(),
          user: z.object({
            id: z.string(),
            username: z.string(),
            token: z.string(),
          }),
        }),
      },
    },
    handler: async (request) => {
      const { username, password } = request.body;
      try {
        // check if user exist
        const user = await fastify.prisma.user.findUnique({
          where: { username },
        });
        if (!user) {
          throw new Error("User not found");
        }
        // verify user password
        const passwordMatch = await verifyPassword(
          password,
          user.password,
          fastify,
        );
        if (!passwordMatch) {
          throw new Error("username or password incorrect");
        }
        // create jwt token
        const token = fastify.jwt.sign({
          id: user.id,
          username: user.username,
        });

        return {
          message: "sign in was a success",
          user: {
            id: user.id,
            username: user.username,
            token,
          },
        };
      } catch (error) {
        if (error instanceof Error) {
          throw new Error(error.message);
        }
      }
    },
  });
};

export default login;
