// RegisterRoute.ts
import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
  location: "/auth/register",
  post: {
    tags: ["auth"],
    summary: "Register a new user",
    requestBody: {
      required: true,
      content: {
        "application/json": {
          schema: {
            type: "object",
            properties: {
              email: { type: "string" },
              first_name: { type: "string" },
              last_name: { type: "string" },
              password: { type: "string" },
            },
            required: ["email", "first_name", "last_name", "password"],
          },
        },
      },
    },
    responses: {
      "201": {
        description: "User registered successfully",
      },
      "400": BadRequestError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
