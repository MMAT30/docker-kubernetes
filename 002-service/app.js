const express = require("express");
const app = express();
const PORT = 8080;

app.get("/", (request, response) => {
  response.send("Testing Hello World!");
});

app.listen(PORT, () => {
  console.log(`Test app listening at http://localhost:${PORT}`);
});
