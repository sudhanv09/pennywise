import { createAuthClient } from "better-auth/solid"
export const authClient = createAuthClient({
    baseURL: import.meta.env.VITE_AUTH_URL
})

export const { signIn, signUp, useSession } = createAuthClient()