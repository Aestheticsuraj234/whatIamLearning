import { z } from "zod";
import { publicProcedure, router } from "./trpc";
import { createHTTPServer } from "@trpc/server/adapters/standalone";

const todoInputType = z.object({
  title: z.string(),
  description: z.string(),
});

const appRouter = router({
  // createTodo: publicProcedure.input(todoInputType).mutation(async (opts) => {
  //   const title = opts.input.title;
  //   const description = opts.input.description;

  //   // do db stuff here .....

  //   return {
  //     id: "1",
  //     title,
  //     description,
  //   };
  // }),
  signUp:publicProcedure.
  input(
    z.object({
      email:z.string(),
      password:z.string(),
  }))
  .mutation(async (opts)=>{
    const username = opts.ctx.username;
    let email = opts.input.email;
    let password = opts.input.password;

    // do database stuff here
    let token = "1232123jsdoilsa"

    return {
      token
    }
  }),
  createTodos:publicProcedure
  .input(
    z.object({
      title:z.string(),
    })
  )
  .mutation(async (opts)=>{
    const username = opts.ctx.username;
    let title = opts.input.title;

    // do database stuff here
    let todo = {
      id:"123",
      title,
    }

    return todo;
  })
});

const server = createHTTPServer({
  router: appRouter,
  createContext(opts){
    let authUser = opts.req.headers.authorization;

    return {
      username:"123"
    }
  }
});

server.listen(3000);

export type AppRouter = typeof appRouter;
