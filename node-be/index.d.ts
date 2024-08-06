declare module "fastify" {
  interface FastifyInstance {
    signUserJwt: <T extends object>(user: T) => string;
    verifyUserJwt: (token: string) => boolean;
  }
}
