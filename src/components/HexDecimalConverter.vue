<template>
    <div>
      <v-row class="align-center">
        <v-col cols="12" md="10">
          <v-text-field
            v-model="inputValue"
            :label="inputLabel"
            :placeholder="inputPlaceholder"
            @input="convertValue"
          />
        </v-col>
        <v-col cols="12" md="2" class="d-flex justify-end">
          <v-btn @click="swapConversion" color="primary">
            Swap 
          </v-btn>
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="12">
          <v-text-field
            v-model="outputValue"
            :label="outputLabel"
            readonly
          />
        </v-col>
      </v-row>
    </div>
  </template>
  
  <script>
  export default {
    name: "HexDecimalConverter",
    data() {
      return {
        conversionMode: "hexToDec", // Default mode is hex to decimal
        inputValue: "",
        outputValue: "",
      };
    },
    computed: {
      inputLabel() {
        return this.conversionMode === "hexToDec" ? "Hexadecimal" : "Decimal";
      },
      outputLabel() {
        return this.conversionMode === "hexToDec" ? "Decimal" : "Hexadecimal";
      },
      inputPlaceholder() {
        return this.conversionMode === "hexToDec"
          ? "Enter hex value (e.g., 1A3F)"
          : "Enter decimal value (e.g., 6703)";
      },
    },
    methods: {
      convertValue() {
        if (this.inputValue === "") {
          this.outputValue = "";
          return;
        }
  
        if (this.conversionMode === "hexToDec") {
          // Convert from hex to decimal
          try {
            this.outputValue = parseInt(this.inputValue, 16).toString(10);
          } catch {
            this.outputValue = "Invalid Hex";
          }
        } else {
          // Convert from decimal to hex
          try {
            this.outputValue = parseInt(this.inputValue, 10).toString(16).toUpperCase();
          } catch {
            this.outputValue = "Invalid Decimal";
          }
        }
      },
      swapConversion() {
        // Toggle between hex-to-dec and dec-to-hex conversion
        this.conversionMode = this.conversionMode === "hexToDec" ? "decToHex" : "hexToDec";
        this.inputValue = "";
        this.outputValue = "";
      },
    },
  };
  </script>
  
  <style scoped>
  .v-text-field {
    margin-bottom: 20px;
  }
  </style>
  