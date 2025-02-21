import { Hono } from "hono";
import { csrf } from "hono/csrf";


const appRouter = new Hono();
appRouter.use(csrf());

// get all transactions
appRouter.get('/transaction', async (c) => {
    return c.text("transaction")
})

// get transaction by param
appRouter.get('/transaction/:id', async (c) => {
    const id = c.req.query('id')
    return c.text("transaction")
})

// add a new transaction
appRouter.post('/transaction/new', async (c) => {
    return c.text("transaction")
})

// delete transaction
appRouter.delete('/transaction/:id', async (c) => {
    return c.text("transaction")
})

// modify transaction by id
appRouter.patch('/transaction/:id', async (c) => {
    return c.text("transaction")
})


export default appRouter;
