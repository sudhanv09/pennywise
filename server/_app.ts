import { Hono } from "hono";
import { csrf } from "hono/csrf";


const app = new Hono();

app.use(csrf());


export const appRouter = app;
    // .route("/", authController)
    // .route("/", todosController);

export type AppRouter = typeof appRouter;
