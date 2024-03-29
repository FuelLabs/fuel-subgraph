export const source = () => `
      startBlock: 0`;

export const mapping = () => `
      specVersion: 0.0.5
      schema:
        file: ./schema.graphql
      dataSources:
        - kind: fuel
          name: fuel-blocks
          network: fuel
          mapping:
            apiVersion: 0.0.6
            language: wasm/assemblyscript
            entities:
              - Block
            blockHandlers:
              - handler: handleBlock
            file: ./src/mapping.ts`;
