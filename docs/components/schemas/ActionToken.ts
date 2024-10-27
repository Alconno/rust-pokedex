import { SchemaObject } from "openapi-ts-builder/dist/types";

export const ActionToken: SchemaObject = {
  id: "ActionToken",
  title: "ActionToken",
  type: "object",
  properties: {
    id: {
      type: "string",
      format: "uuid",
    },
    entity_id: {
      type: "string",
      format: "uuid",
    },
    token: {
      type: "string",
    },
    action_name: {
      type: "string",
    },
    created_at: {
      type: "string",
      format: "date-time",
    },
    updated_at: {
      type: "string",
      format: "date-time",
    },
    expires_at: {
      type: "string",
      format: "date-time",
    },
    executed_at: {
      anyOf: [
        { type: "string", format: "date-time" },
        { type: "null" }
      ]
    },
  },
  required: ["id", "entity_id", "token", "action_name", "created_at", "updated_at", "expires_at"],
};