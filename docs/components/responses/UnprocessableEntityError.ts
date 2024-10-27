import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "UnprocessableEntityError",
  description: "Error when validation of the request fails", // 422
  content: {
    "application/json": {
      schema: {
        type: "object",
        properties: {
          errors: {
            type: "object",
            properties: {
              fieldNameThatFailsValidation: {
                description: "Object for fieldName that is failing validation",
                type: "object",
                properties: {
                  field: {
                    type: "string",
                    description: "Name of the field that fails validation",
                  },
                  errors: {
                    type: "array",
                    description: "Array of strings that are actual error codes",
                    items: {
                      type: "string",
                    },
                  },
                },
              },
            },
          },
        },
      },
      example: {
        errors: {
          fieldName: {
            field: "fieldName",
            errors: ["required"],
          },
          anotherFieldName: {
            field: "anotherFieldName",
            errors: ["length_min:3"],
          },
        },
      },
    },
  },
} as ResponseObject;

