import { z } from "zod";
import { publicProcedure, router } from "./trpc";

const todoInputType = z.object({
  title: z.string(),
  description: z.string(),
});

const appRouter = router({
  createTodo: publicProcedure.input(todoInputType).mutation(async (opts) => {
    const title = opts.input.title;
    const description = opts.input.description;

    // do db stuff here .....

    return {
        id:"1",
      title,
      description,
    }
  }),
});

export type AppRouter = typeof appRouter;
