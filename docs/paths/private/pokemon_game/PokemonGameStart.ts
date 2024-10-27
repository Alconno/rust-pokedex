import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../components/responses/UnauthorizedError";

export default {
  location: "/api/pokedex/{version}/pokemon",
  get: {
    tags: ["pokemon"],
    summary: "Start a new Pokemon game",
    responses: {
      "200": {
        description: "Pokemon game started successfully",
        content: {
          "application/json": {
            schema: {
              type: "object",
              properties: {
                pokemon_image: { type: "string" },
                attempt_id: { type: "string" },
                pokemon_id: { type: "integer" },
              },
              required: ["pokemon_image", "attempt_id", "pokemon_id"],
            },
          },
        },
      },
      "401": UnauthorizedError,
      "500": InternalServerError,
    },
  },
} as PathItemObject;
