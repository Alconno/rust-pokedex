import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../components/responses/InternalServerError";
import BadRequestError from "../../../components/responses/BadRequestError";
import UnauthorizedError from "../../../components/responses/UnauthorizedError";

export default {
  location: "/api/pokedex/{version}/pokemon/attempt/{attempt_id}",
  post: {
    tags: ["pokemon"],
    summary: "Submit a guess attempt for a Pokemon",
    requestBody: {
      required: true,
      content: {
        "application/json": {
          schema: {
            type: "object",
            properties: {
              guess: { type: "string" },
            },
            required: ["guess"],
          },
        },
      },
    },
    responses: {
      "200": {
        description: "Guess attempt successful",
        content: {
          "application/json": {
            schema: {
              type: "string",
            },
          },
        },
      },
      "400": BadRequestError,
      "401": UnauthorizedError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
