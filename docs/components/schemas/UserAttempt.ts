import { SchemaObject } from "openapi-ts-builder/dist/types";

export const UserAttempt: SchemaObject = {
    id: "UserAttempt",
    title: "UserAttempt",
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
      is_successful: {
        type: "boolean",
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
    required: ["id", "user_id", "pokemon_id", "is_successful", "created_at", "updated_at"],
  };