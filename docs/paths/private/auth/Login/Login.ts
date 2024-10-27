import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
  location: "/auth/login",
  post: {
    tags: ["auth"],
    summary: "Authenticate user",
    requestBody: {
      required: true,
      content: {
        "application/json": {
          schema: {
            type: "object",
            properties: {
              email: { type: "string" },
              password: { type: "string" },
            },
            required: ["email", "password"],
          },
        },
      },
    },
    responses: {
      "200": {
        description: "User authenticated successfully",
        headers: {
          "Set-Cookie": {
            description: "Cookie containing user session data",
            schema: { type: "string" }, // Define cookie schema here
          },
        },
      },
      "400": BadRequestError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
