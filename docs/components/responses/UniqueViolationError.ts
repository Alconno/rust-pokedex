import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "UniqueViolationError",
  description: "Request could not be processed because of conflict in the request", // 409
  content: {
    "application/json": {
      schema: {
        type: "object",
        properties: {
          message: {
            type: "string",
            nullable: false,
          },
          code: {
            type: "string",
            nullable: false,
          },
          cause: {
            type: "string",
            nullable: true,
          },
          payload: {
            nullable: true,
            type: "object",
            properties: {
              "{some_key}": {
                type: "string",
                nullable: false,
              },
            },
          },
        },
      },
      example: {
        message: "Field with that value already exists",
        code: "unique_violation",
        cause: "Expertise with that name already exists",
        payload: null
    },
    },
  },
} as ResponseObject;
