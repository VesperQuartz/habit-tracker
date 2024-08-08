import { AutoloadPluginOptions } from "@fastify/autoload";
import { FastifyPluginAsync, FastifyServerOptions } from "fastify";
export interface AppOptions extends FastifyServerOptions, Partial<AutoloadPluginOptions> {
}
declare const options: AppOptions;
declare const app: FastifyPluginAsync<AppOptions>;
export default app;
export { app, options };
