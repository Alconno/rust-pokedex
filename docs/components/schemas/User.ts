import { SchemaObject } from "openapi-ts-builder/dist/types";

export const User: SchemaObject = {
    id: "User",
    title: "User",
    type: "object",
    properties: {
      id: {
        type: "string",
        format: "uuid",
      },
      email: {
        type: "string",
      },
      first_name: {
        type: "string",
      },
      last_name: {
        type: "string",
      },
      password: {
        type: "string",
      },
      email_verified_at: {
        anyOf: [
          { type: "string", format: "date-time" },
          { type: "null" }
        ]
      },
      refresh_token: {
        anyOf: [
          { type: "string" },
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
    required: ["id", "email", "first_name", "last_name", "password", "created_at", "updated_at"],
  };