import { join } from "path";
import AutoLoad, { AutoloadPluginOptions } from "@fastify/autoload";
import { FastifyPluginAsync, FastifyServerOptions } from "fastify";
import fastifySwagger from "@fastify/swagger";
import fastifySwaggerUI from "@fastify/swagger-ui";
import {
  jsonSchemaTransform,
  serializerCompiler,
  validatorCompiler,
} from "fastify-type-provider-zod";
import("@mgcrea/pino-pretty-compact");

export interface AppOptions
  extends FastifyServerOptions,
    Partial<AutoloadPluginOptions> {}
// Pass --options via CLI arguments in command to enable these options.
const options: AppOptions = {
  logger: {
    level: "debug",
    transport: {
      target: "@mgcrea/pino-pretty-compact",
      options: {
        translateTime: "HH:MM:ss Z",
        ignore: "pid,hostname",
      },
    },
  },
};

const app: FastifyPluginAsync<AppOptions> = async (
  fastify,
  opts,
): Promise<void> => {
  // Place here your custom code!
  fastify.setValidatorCompiler(validatorCompiler);
  fastify.setSerializerCompiler(serializerCompiler);

  fastify.register(fastifySwagger, {
    openapi: {
      info: {
        title: "Habit Tracker",
        description:
          "The Habit Tracker is a simple tool that helps you build and maintain positive habits by tracking your progress.",
        version: "1.0.0",
      },
      servers: [],
    },
    transform: jsonSchemaTransform,
    // You can also create transform with custom skiplist of endpoints that should not be included in the specification:
    //
    // transform: createJsonSchemaTransform({
    //   skipList: [ '/documentation/static/*' ]
    // })
  });

  fastify.register(fastifySwaggerUI, {
    routePrefix: "/docs",
  });
  // Do not touch the following lines

  // This loads all plugins defined in plugins
  // those should be support plugins that are reused
  // through your application
  void fastify.register(AutoLoad, {
    dir: join(__dirname, "plugins"),
    options: opts,
  });

  // This loads all plugins defined in routes
  // define your routes in one of these
  void fastify.register(AutoLoad, {
    dir: join(__dirname, "routes"),
    options: opts,
  });
};

export default app;
export { app, options };
