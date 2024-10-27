import { Builder } from "openapi-ts-builder";
import * as path from "path";

export default Builder.create({
  title: "Pokemon game",
  version: "1.0.0",
})
  .addServer({
    url: "http://localhost:8080",
    description: "Local development instance"
  })
  .addPathsDir(path.resolve(__dirname, "paths"))
  .addComponentsDir(path.resolve(__dirname, "components"));
