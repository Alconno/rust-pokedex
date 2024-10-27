import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "UnauthorizedError",
  description: "Authorization error", // 401
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
            enum: [
              {
                account_deactivated:
                  "account has been soft deleted/user is banned",
              },
              {
                email_not_verified:
                  "email of the account has not been verified",
              },
              {
                invalid_credentials: "user or password not ok",
              },
              {
                invalid_otp_token: "otp token is invalid",
              },
              {
                no_session: "no session found and it was expected to be there",
              },
              {
                authentication_failed_no_session_found:
                  "no session found in cache",
              },
              {
                oauth_google_invalid_authorization_code:
                  "code given from the oauth provider is invalid",
              },
              {
                oauth_github_invalid_authorization_code:
                  "code given from the oauth provider is invalid",
              },
              {
                authentication_failed_no_intermediate_session:
                  "no intermediate session found and it was expected to be there (first step of login skipped)",
              },
              {
                invalid_one_time_password:
                  "one time password is invalid (token sent to user after login if 2fa is enabled)"
              },
            ],
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
        message: "Unauthorized error",
        code: "unauthorized",
        cause: "authentication_error_invalid_csrf",
        payload: null,
      },
    },
  },
} as ResponseObject;

