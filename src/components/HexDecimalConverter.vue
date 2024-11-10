<template>
  <v-card class="pa-5">
    <div class="cont">
      <v-text-field v-model="inputValue" :label="inputLabel" :placeholder="inputPlaceholder" @input="convertValue"
        class="mb-4"/>
      <v-btn @click="swapConversion" color="primary" class="btn">
        Swap
      </v-btn>
    </div>
    <v-text-field v-model="outputValue" :label="outputLabel" readonly outlined class="mt-4" />
  </v-card>
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
.mb-4 {
  width: 4rem;
}
.cont {
  display: flex;
}
.btn{
  margin-left: 1rem;
  top:0.7rem;
}
</style>
