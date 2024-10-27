import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../components/responses/InternalServerError";
import PayloadTooLargeError from "../../components/responses/PayloadTooLargeError";
import TooManyRequestsError from "../../components/responses/TooManyRequestsError";
import UnsupportedMediaTypeError from "../../components/responses/UnsupportedMediaTypeError";

export default {
    location: "/_health",
    get: {
    tags: ["public"],
    security: [],
    summary: "Check the server's health",
    responses: {
      "200": {
            description: "Ok",
            content: {
                "text/plain": {
                    schema: {
                        type: "string",
                        example: "I am healthy!" 
                    }
                }
            },
        },  
        "413": PayloadTooLargeError,
        "415": UnsupportedMediaTypeError,
        "429": TooManyRequestsError,
        "500": InternalServerError,
        },
    },
} as PathItemObject;

