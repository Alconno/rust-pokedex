import { SchemaObject } from "openapi-ts-builder/dist/types";

export const UserPokedex: SchemaObject = {
    id: "UserPokedex",
    title: "UserPokedex",
    type: "object",
    properties: {
      id: {
        type: "string",
        format: "uuid",
      },
      user_id: {
        type: "string",
        format: "uuid",
      },
      pokemon_id: {
        type: "string",
        format: "uuid",
      },
      created_at: {
        type: "string",
        format: "date-time",
      },
      updated_at: {
        type: "string",
        format: "date-time",
      },
    },
    required: ["id", "user_id", "pokemon_id", "created_at", "updated_at"],
  };