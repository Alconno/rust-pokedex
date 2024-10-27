import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";

export default {
  location: "/auth/refresh",
  post: {
    tags: ["auth"],
    summary: "Refresh user token",
    responses: {
      "200": {
        description: "Token refreshed successfully",
        headers: {
          "Set-Cookie": {
            description: "Cookie containing updated token",
            schema: { type: "string" }, // Define cookie schema here
          },
        },
      },
      "401": UnauthorizedError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
