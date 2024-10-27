// VerifyEmailRoute.ts
import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
  location: "/auth/verify-email/{action_token}",
  get: {
    tags: ["auth"],
    summary: "Verify email using action token",
    parameters: [
      {
        name: "action_token",
        in: "path",
        required: true,
        schema: {
          type: "string",
        },
      },
    ],
    responses: {
      "302": {
        description: "Redirect to login page",
      },
      "400": BadRequestError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
