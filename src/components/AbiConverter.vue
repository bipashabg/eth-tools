<template>
    <v-card class="pa-5">
      <h3>ABI Encoder</h3>
  
      <v-text-field
        label="ABI Types (comma-separated)"
        placeholder="e.g., address,uint256,string"
        v-model="abiTypes"
        outlined
      ></v-text-field>
  
      <v-textarea
        label="Parameters (comma-separated)"
        placeholder="e.g., '0xabc123...', '123', 'Hello World'"
        v-model="parameters"
        outlined
        rows="3"
      ></v-textarea>
  
      <v-btn color="#865DFF" @click="encodeAbi">Encode</v-btn>
        <br />
        <br />
      <v-textarea
        label="Encoded Output"
        :value="encodedOutput"
        outlined
        rows="5"
        readonly
      ></v-textarea>
    </v-card>
  </template>
  
  <script>
  import { ethers } from "ethers";
  
  export default {
    name: "AbiConverter",
    data() {
      return {
        abiTypes: "",
        parameters: "",
        encodedOutput: ""
      };
    },
    methods: {
      encodeAbi() {
        try {
          // Convert the ABI types and parameters from strings to arrays
          const typesArray = this.abiTypes.split(",").map((type) => type.trim());
          const parametersArray = this.parameters.split(",").map((param) => param.trim());
  
          // Use ethers.js to encode the ABI
          this.encodedOutput = ethers.utils.defaultAbiCoder.encode(typesArray, parametersArray);
        } catch (error) {
          this.encodedOutput = `Error: ${error.message}`;
        }
      }
    }
  };
  </script>
  
  <style scoped>
  h3 {
    font-weight: bold;
    margin-bottom: 20px;
    font-family: 'Dongle', sans-serif;
  }
  </style>
  