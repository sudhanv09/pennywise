import * as v from "valibot";

export const accountSchema = v.object({
  name: v.string(),
  currency: v.string(),
});