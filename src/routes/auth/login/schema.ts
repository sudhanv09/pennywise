import * as v from "valibot";

export const loginSchema = v.object({
    email: v.pipe(v.string(), v.email("Please enter a valid email address")),
    password: v.pipe(
        v.string(),
        v.minLength(8, "Password must be at least 8 characters long")
    ),
});
