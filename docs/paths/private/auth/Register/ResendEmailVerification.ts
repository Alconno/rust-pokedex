// ResendEmailRoute.ts
import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import PayloadTooLargeError from "../../../../components/responses/PayloadTooLargeError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
  location: "/auth/verify-email/resend",
  post: {
    tags: ["auth"],
    summary: "Resend email for verification",
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
      "201": {
        description: "Email resent successfully",
      },
      "400": BadRequestError,
      "413": PayloadTooLargeError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
