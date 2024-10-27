import { SchemaObject } from "openapi-ts-builder/dist/types";

export const Pokemon: SchemaObject = {
    id: "Pokemon",
    title: "Pokemon",
    type: "object",
    properties: {
      id: {
        type: "string",
        format: "uuid",
      },
      name: {
        anyOf: [
          { type: "string" },
          { type: "null" }
        ]
      },
      base_experience: {
        anyOf: [
          { type: "integer" },
          { type: "null" }
        ]
      },
      height: {
        anyOf: [
          { type: "integer" },
          { type: "null" }
        ]
      },
      pokemon_id: {
        anyOf: [
          { type: "integer" },
          { type: "null" }
        ]
      },
      is_default: {
        anyOf: [
          { type: "boolean" },
          { type: "null" }
        ]
      },
      order: {
        anyOf: [
          { type: "integer" },
          { type: "null" }
        ]
      },
      image: {
        anyOf: [
          { type: "string" },
          { type: "null" }
        ]
      },
      weight: {
        anyOf: [
          { type: "integer" },
          { type: "null" }
        ]
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
    required: ["id", "created_at", "updated_at"],
  };