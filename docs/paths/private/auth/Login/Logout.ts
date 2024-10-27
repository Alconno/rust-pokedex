import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";

export default {
  location: "/auth/logout",
  post: {
    tags: ["auth"],
    summary: "Logout user",
    responses: {
      "200": {
        description: "User logged out successfully",
        headers: {
          "Set-Cookie": {
            description: "Cookie with empty value to remove user session",
            schema: { type: "string" }, // Define cookie schema here
          },
        },
      },
      "401": UnauthorizedError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
