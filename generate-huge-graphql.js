const fs = require("fs");
const path = require("path");

function generateSchema(typeCount, fieldCount) {
  let schema = `# Auto-generated large GraphQL schema\n\n`;

  schema += `type Query {\n`;
  for (let i = 1; i <= typeCount; i++) {
    schema += `  type${i}(id: ID!): Type${i}\n`;
  }
  schema += `}\n\n`;

  for (let i = 1; i <= typeCount; i++) {
    schema += `type Type${i} {\n`;
    for (let j = 1; j <= fieldCount; j++) {
      schema += `  field${j}: String\n`;
    }
    if (i < typeCount) {
      schema += `  nestedType: Type${i + 1}\n`; // Linking to next type for nesting
    }
    schema += `}\n\n`;
  }

  return schema;
}

function generateQuery(typeCount, depth) {
  let query = `# Auto-generated large GraphQL query\n\nquery LargeQuery {\n`;

  function addFields(type, currentDepth, indentLevel) {
    let indent = "  ".repeat(indentLevel);
    query += `${indent}${type}(id: "1") {\n`;
    for (let j = 1; j <= 100; j++) {
      query += `${indent}  field${j}\n`;
    }
    if (currentDepth < depth) {
      query += `${indent}  nestedType {\n`;
      addFields(
        `type${parseInt(type.match(/\d+/)[0]) + 1}`,
        currentDepth + 1,
        indentLevel + 2
      );
      query += `${indent}  }\n`;
    }
    query += `${indent}}\n`;
  }

  addFields(`type1`, 1, 1);

  query += `}\n`;
  return query;
}

const typeCount = 500;
const fieldCount = 10;
const depth = 50;

const schema = generateSchema(typeCount, fieldCount);
const query = generateQuery(typeCount, depth);

const schemaFilePath = path.join(__dirname, "fixtures/schemas", "huge.graphql");
const queryFilePath = path.join(__dirname, "fixtures/queries", "huge.graphql");

fs.writeFileSync(schemaFilePath, schema, "utf8");
console.log(`Schema generated and saved to ${schemaFilePath}`);

fs.writeFileSync(queryFilePath, query, "utf8");
console.log(`Query generated and saved to ${queryFilePath}`);
