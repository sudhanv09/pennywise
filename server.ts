import { Hono } from "hono";

import { renderPage } from "vike/server";
import appRouter from "./server/_app";

const app = new Hono();
const PORT = 3000;

app.get("/up", async (c) => {
  return c.newResponse("🟢 UP", { status: 200 });
});

app.route("/api/*", appRouter);

app.get("*", async (c, next) => {
  const pageContextInit = {
    urlOriginal: c.req.url,
    request: c.req,
    response: c.res,
  };
  const pageContext = await renderPage(pageContextInit);
  const { httpResponse } = pageContext;
  if (!httpResponse) {
    return next();
  } else {
    const { body, statusCode, headers } = httpResponse;
    headers.forEach(([name, value]) => c.header(name, value));
    c.status(statusCode);

    return c.body(body);
  }
});

app.onError((_, c) => {
  return c.json(
    {
      error: {
        message: c.error?.message ?? "Something went wrong.",
      },
    },
    500
  );
});

console.log("Running at http://localhost:" + PORT);

export default {
  port: PORT,
  fetch: app.fetch,
};
